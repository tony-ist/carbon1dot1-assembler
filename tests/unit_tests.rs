use carbon1dot1_assembler::{
    ast::*,
    lower_labels::lower_labels,
    name_mangling::mangle,
    parser::parse,
};

#[test]
fn test_name_mangling_simple_function() {
    let source = r#"
nop
FUNC .test_func
inc r1
.inner_label
dec r2
END
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    
    // Should have: nop, hlt, .test_func label, inc r1, inner_label, dec r2
    assert_eq!(mangled.len(), 6);
    
    // Check that function label is preserved
    if let FuncBody::Label(ref label) = mangled[2] {
        assert_eq!(label, ".test_func");
    } else {
        panic!("Expected function label");
    }
    
    // Check that inner label is mangled
    if let FuncBody::Label(ref label) = mangled[4] {
        assert_eq!(label, ".__INNER_FUNC_LABEL.test_func_.inner_label");
    } else {
        panic!("Expected mangled inner label");
    }
}

#[test]
fn test_name_mangling_with_function_calls() {
    let source = r#"
cal .my_func
FUNC .my_func
ret
END
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    
    // Check that function call uses unmangled label
    if let FuncBody::Instruction(ref instr) = mangled[0] {
        assert_eq!(instr.opcode, Opcode::Cal);
        if let Operand::Label(ref label) = instr.operands[0] {
            assert_eq!(label, ".my_func");
        } else {
            panic!("Expected label operand");
        }
    }
}

#[test]
fn test_name_mangling_preserves_main_labels() {
    let source = r#"
.main_label
nop
brc jmp .main_label
FUNC .func
.func_label
ret
END
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    
    // Main labels should not be mangled
    if let FuncBody::Label(ref label) = mangled[0] {
        assert_eq!(label, ".main_label");
    }
    
    // Function internal labels should be mangled
    let mut found_mangled_label = false;
    for item in &mangled {
        if let FuncBody::Label(ref label) = item {
            if label.contains(".__INNER_FUNC_LABEL") {
                found_mangled_label = true;
                assert_eq!(label, ".__INNER_FUNC_LABEL.func_.func_label");
            }
        }
    }
    assert!(found_mangled_label, "Should have found mangled function label");
}

#[test]
fn test_label_lowering_simple() {
    let source = r#"
.start
nop
brc jmp .start
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    
    // Should have: nop, brc instruction, hlt
    assert_eq!(lowered.len(), 3);
    
    // Check that brc instruction has immediate operand (address 0)
    if let FuncBody::Instruction(ref instr) = lowered[1] {
        assert_eq!(instr.opcode, Opcode::Brc);
        if let Operand::Immediate(addr) = instr.operands[1] {
            assert_eq!(addr, 0); // .start is at address 0
        } else {
            panic!("Expected immediate operand for lowered label");
        }
    }
}

#[test]
fn test_label_lowering_complex() {
    let source = r#"
nop          // Address 0 (1 byte)
inc r1       // Address 1 (1 byte)  
.target      // Address 2
dec r2       // Address 2 (1 byte)
brc jmp .target  // Address 3 (3 bytes: high, low, opcode)
lim r0 42    // Address 6 (2 bytes: opcode+reg, immediate)
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    
    // Find the brc instruction and check its target
    for item in &lowered {
        if let FuncBody::Instruction(ref instr) = item {
            if instr.opcode == Opcode::Brc {
                if let Operand::Immediate(addr) = instr.operands[1] {
                    assert_eq!(addr, 2); // .target is at address 2
                } else {
                    panic!("Expected immediate operand for brc target");
                }
            }
        }
    }
}

#[test]
fn test_label_lowering_removes_labels() {
    let source = r#"
.label1
nop
.label2  
inc r1
.label3
dec r2
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    
    // Labels should be removed, only instructions remain
    for item in &lowered {
        match item {
            FuncBody::Instruction(_) | FuncBody::Data(_) => {
                // These are expected
            }
            FuncBody::Label(_) => {
                panic!("Labels should be removed by label lowering");
            }
        }
    }
}

#[test]
fn test_instruction_size_calculation() {
    let source = r#"
nop              // 1 byte
inc r1           // 1 byte  
lim r2 42        // 2 bytes (opcode+reg, immediate)
brc jmp .target  // 3 bytes (high, low, opcode) 
.target
psi $0 123       // 2 bytes (opcode+addr, immediate)
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    
    // .target should be at address 1 + 1 + 2 + 3 = 7
    for item in &lowered {
        if let FuncBody::Instruction(ref instr) = item {
            if instr.opcode == Opcode::Brc {
                if let Operand::Immediate(addr) = instr.operands[1] {
                    assert_eq!(addr, 7); // .target calculated address
                } else {
                    panic!("Expected immediate operand");
                }
            }
        }
    }
}

#[test]
fn test_data_affects_addresses() {
    let source = r#"
nop              // Address 0 (1 byte)
"hello"          // Address 1 (5 bytes)
.target          // Address 6
inc r1           // Address 6 (1 byte)
brc jmp .target  // Address 7 (3 bytes)
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    
    // Check that .target resolves to address 6 (after nop + "hello")
    for item in &lowered {
        if let FuncBody::Instruction(ref instr) = item {
            if instr.opcode == Opcode::Brc {
                if let Operand::Immediate(addr) = instr.operands[1] {
                    assert_eq!(addr, 6);
                } else {
                    panic!("Expected immediate operand");
                }
            }
        }
    }
}

#[test]
fn test_multiple_functions_addressing() {
    let source = r#"
nop              // Main: address 0
FUNC .func1      // Starts after main + hlt
.loop1           // Function label
ret              
END
FUNC .func2
.loop2           // Function label  
ret
END
"#;
    let ast = parse(source, "test.s");
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    
    // Verify that all labels are properly resolved without panicking
    // This mainly tests that the label lowering doesn't crash with multiple functions
    assert!(lowered.len() > 0);
    
    // All instructions should have resolved operands (no labels remaining)
    for item in &lowered {
        if let FuncBody::Instruction(ref instr) = item {
            for operand in &instr.operands {
                match operand {
                    Operand::Label(_) => {
                        panic!("Found unresolved label: {:?}", operand);
                    }
                    _ => {
                        // Other operand types are fine
                    }
                }
            }
        }
    }
}