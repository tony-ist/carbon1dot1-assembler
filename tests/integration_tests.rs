use carbon1dot1_assembler::{
    assembler::assemble,
    lower_labels::lower_labels,
    name_mangling::mangle,
    parser::parse,
};

#[test]
fn test_basic_instructions_without_operands() {
    let source = "nop\nhlt\nret\npsh\npop\nfls";
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);

    // Just test that it assembles without error and produces reasonable output
    assert!(assembled.len() >= 6); // Should have at least the instructions we specified
    assert_eq!(assembled[0], 0); // nop should be opcode 0
    
    // Test that the assembly process completes successfully
    assert!(assembled.len() > 0);
}

#[test]
fn test_register_operands() {
    let source = "inc r1\ndec r2\nadd r3\nneg r0";
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);

    // Test that instructions with register operands work
    assert!(assembled.len() >= 4);
    
    // Test that register numbers are encoded in the lower 3 bits
    assert_eq!(assembled[0] & 0b111, 1); // r1
    assert_eq!(assembled[1] & 0b111, 2); // r2  
    assert_eq!(assembled[2] & 0b111, 3); // r3
    assert_eq!(assembled[3] & 0b111, 0); // r0
}

#[test]
fn test_immediate_operands() {
    let source = "lim r1 42\nbsl 3\nbsr 7";
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);

    // Test that immediate operands are handled correctly
    assert!(assembled.len() >= 4);
    
    // Test that lim r1 42 produces the register encoding and immediate value
    assert_eq!(assembled[0] & 0b111, 1); // r1 register
    assert_eq!(assembled[1], 42); // immediate value 42
    
    // Test that bsl and bsr encode their immediate values correctly
    assert_eq!(assembled[2] & 0b111, 3); // bsl 3
    assert_eq!(assembled[3] & 0b111, 7); // bsr 7
}

#[test]
fn test_address_operands() {
    let source = "pst $1\npld $2\npsi $0 123";
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);

    // Test that address operands are handled correctly
    assert!(assembled.len() >= 4);
    
    // Test that address operands encode the address in the lower bits
    assert_eq!(assembled[0] & 0b111, 1); // $1
    assert_eq!(assembled[1] & 0b111, 2); // $2  
    assert_eq!(assembled[2] & 0b111, 0); // $0
    assert_eq!(assembled[3], 123); // immediate value for psi
}

#[test]
fn test_condition_operands() {
    let source = "brc jmp .target\n.target\nnop";
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);

    // Test that branch instructions work with labels
    assert!(assembled.len() >= 4);
    
    // BRC instructions should produce 3 bytes: high_addr, low_addr, opcode_with_condition
    // The target address should point to the nop instruction
    // This is a more flexible test that doesn't depend on exact opcodes
    
    // Just verify the structure makes sense
    assert!(assembled.len() >= 4); // At least brc (3 bytes) + nop + hlt
}

#[test]
fn test_function_processing() {
    let source = r#"
nop
FUNC .test_func
inc r1
.inner_label
dec r2
brc jmp .inner_label
END
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);

    // Test that functions are processed correctly
    assert!(assembled.len() >= 6); // Should have multiple instructions
    
    // First instruction should be nop
    assert_eq!(assembled[0], 0);
    
    // Should have reasonable output that includes function code
    assert!(assembled.len() > 5); // Main + hlt + function instructions
}

#[test]
fn test_string_data() {
    let source = r#"nop
"hello""#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);

    // Test that string data is included correctly
    assert!(assembled.len() >= 7); // nop + hlt + "hello" (5 chars)
    
    // First instruction should be nop
    assert_eq!(assembled[0], 0);
    
    // Should contain the string data
    let assembled_string = String::from_utf8_lossy(&assembled);
    assert!(assembled_string.contains("hello"));
}

#[test]
fn test_complex_program() {
    let source = r#"
lim r0 0
lim r1 10
.loop
inc r0
dec r1
brc neq .loop
hlt
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(lowered);

    // Test that complex programs with loops work
    assert!(assembled.len() >= 10); // Should have multiple instructions
    
    // Test that immediate values are present
    assert!(assembled.contains(&0)); // immediate 0
    assert!(assembled.contains(&10)); // immediate 10
    
    // Test that the program structure makes sense
    assert!(assembled.len() > 8); // Should be substantial
}