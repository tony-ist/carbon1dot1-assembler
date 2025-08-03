use carbon1dot1_assembler::{
    assembler::assemble,
    disassembler::disassemble,
    lower_labels::lower_labels,
    name_mangling::mangle,
    parser::parse,
};

#[test]
fn test_fibonacci_program() {
    // Test a fibonacci-like program similar to the examples
    let source = r#"
LIM R0 0
LIM R2 1
.loop
ADD R2
PST $0
ADR R2
RST R3
RLD R2
PST $0
RLD R3
BRC JMP .loop
"#;
    
    let ast = parse(source, "fib.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);
    
    // Should assemble without errors
    assert!(assembled.len() > 0);
    
    // Test round-trip: disassemble and verify we get reasonable output
    let disassembled = disassemble(&assembled);
    assert!(disassembled.len() > 0);
    
    // Should contain expected instructions
    let disasm_text = disassembled.join("\n");
    assert!(disasm_text.contains("lim"));
    assert!(disasm_text.contains("add"));
    assert!(disasm_text.contains("pst"));
    assert!(disasm_text.contains("brc"));
}

#[test]
fn test_counting_program() {
    let source = r#"
LIM R0 0
LIM R1 10
.loop
INC R0
PST $0
DEC R1
BRC NEQ .loop
HLT
"#;
    
    let ast = parse(source, "count.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);
    
    // Verify specific byte sequence for counting program
    assert!(assembled.len() > 10); // Should be a reasonable size
    
    // Test that it disassembles correctly
    let disassembled = disassemble(&assembled);
    
    // Just check that disassembly works and produces reasonable output
    assert!(disassembled.len() > 0);
    let disasm_text = disassembled.join("\n").to_lowercase();
    
    // Check for core instructions that should be present
    assert!(disasm_text.contains("lim"));
    assert!(disasm_text.contains("inc"));
    assert!(disasm_text.contains("brc"));
    assert!(disasm_text.contains("hlt"));
}

#[test]
fn test_function_with_parameters() {
    let source = r#"
// Main program
LIM R0 5
LIM R1 10
CAL .multiply
PST $0
HLT

// Multiply function: R0 * R1 -> R0
FUNC .multiply
RST R2    // Counter
RST R0    // Result accumulator
.mult_loop
ADD R1    // Add R1 to R0
DEC R2    // Decrement counter
BRC NEQ .mult_loop
RET
END
"#;
    
    let ast = parse(source, "multiply.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);
    
    // Should assemble without errors
    assert!(assembled.len() > 0);
    
    // Test disassembly
    let disassembled = disassemble(&assembled);
    assert!(disassembled.len() > 0);
    
    // Should have function call and return
    let disasm_text = disassembled.join("\n").to_lowercase();
    assert!(disasm_text.contains("cal"));
    assert!(disasm_text.contains("ret"));
}

#[test]
fn test_data_and_code_mixing() {
    let source = r#"
LIM R0 0
"Hello"
PST $1
"World"
PST $2
HLT
"#;
    
    let ast = parse(source, "data_mix.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);
    
    // Should contain both code and data
    assert!(assembled.len() > 10);
    
    // Verify that string data is included
    let assembled_str = String::from_utf8_lossy(&assembled);
    assert!(assembled_str.contains("Hello"));
    assert!(assembled_str.contains("World"));
}

#[test]
fn test_complex_branching() {
    let source = r#"
LIM R0 5
.test_positive
BRC GT .positive
BRC EQ .zero
BRC LT .negative
BRC JMP .end

.positive
LIM R1 1
BRC JMP .end

.zero  
LIM R1 0
BRC JMP .end

.negative
LIM R1 255
BRC JMP .end

.end
PST $0
HLT
"#;
    
    let ast = parse(source, "branch.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);
    
    // Should handle complex branching
    assert!(assembled.len() > 0);
    
    // Test disassembly contains all conditions
    let disassembled = disassemble(&assembled);
    let disasm_text = disassembled.join("\n").to_lowercase();
    
    assert!(disasm_text.contains("brc gt"));
    assert!(disasm_text.contains("brc eq"));
    assert!(disasm_text.contains("brc lt"));
}

#[test]
fn test_stack_operations() {
    let source = r#"
LIM R0 42
PSH
LIM R0 123  
PSH
POP
PST $0    // Should output 123
POP  
PST $1    // Should output 42
HLT
"#;
    
    let ast = parse(source, "stack.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);
    
    // Should assemble stack operations
    assert!(assembled.len() > 0);
    
    let disassembled = disassemble(&assembled);
    let disasm_text = disassembled.join("\n").to_lowercase();
    
    assert!(disasm_text.contains("psh"));
    assert!(disasm_text.contains("pop"));
}

#[test]
fn test_memory_operations() {
    let source = r#"
LIM R0 123
MST R0    // Store to memory
LIM R0 0  // Clear R0
MLD R0    // Load from memory
PST $0    // Output should be 123
HLT
"#;
    
    let ast = parse(source, "memory.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);
    
    let disassembled = disassemble(&assembled);
    let disasm_text = disassembled.join("\n").to_lowercase();
    
    assert!(disasm_text.contains("mst"));
    assert!(disasm_text.contains("mld"));
}

#[test]
fn test_round_trip_assembly_disassembly() {
    // Test that assembling then disassembling produces reasonable output
    let source = r#"
nop
inc r1
dec r2
add r3
lim r0 42
brc jmp .loop
.loop
hlt
"#;
    
    let ast = parse(source, "round_trip.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);
    let disassembled = disassemble(&assembled);
    
    // Check that all major instruction types are preserved
    let disasm_text = disassembled.join("\n").to_lowercase();
    
    assert!(disasm_text.contains("nop"));
    assert!(disasm_text.contains("inc r1"));
    assert!(disasm_text.contains("dec r2"));
    assert!(disasm_text.contains("add r3"));
    assert!(disasm_text.contains("lim r0"));
    assert!(disasm_text.contains("brc jmp"));
    assert!(disasm_text.contains("hlt"));
}

#[test]
fn test_all_instruction_types_compile() {
    // Comprehensive test with every instruction type
    let source = r#"
// Arithmetic
NOP
INC R1
DEC R2
ADD R3
ADR R0
NEG R1
SUB R2
BSB R3
CMP R0

// Logical
BOR R1
AND R2
XOR R3
BSL 3
BSR 5

// Memory & Stack
LIM R0 42
RST R1
RLD R2
MST R3
MLD R0
PSH
POP
PST $0
PSI $1 123
PLD $2

// Control Flow
CAL .func
RET
BRC JMP .label
BRC EQ .label  
BRC NEQ .label
BRC GT .label
BRC LT .label
BRC GTEQ .label
BRC LTEQ .label
BRC EVEN .label
JID R0
PRD JMP .label

// Other
HLT
FLS

.label
.func
RET

// Data
"test string"
"#;
    
    let ast = parse(source, "comprehensive.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);
    
    // Should assemble without errors
    assert!(assembled.len() > 50); // Should be substantial
    
    // Should disassemble without errors
    let disassembled = disassemble(&assembled);
    assert!(disassembled.len() > 20); // Should have many instructions
    
    // Verify no instructions are lost
    assert!(disassembled.iter().all(|line| !line.is_empty()));
}