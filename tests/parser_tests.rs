use carbon1dot1_assembler::{ast::*, parser::parse};

#[test]
fn test_parse_basic_instructions() {
    let source = "nop\ninc r1\ndec r2";
    let ast = parse(source, "test.s");
    
    assert_eq!(ast.instrs.len(), 3);
    
    // Check nop instruction
    if let FuncBody::Instruction(ref instr) = ast.instrs[0] {
        assert_eq!(instr.opcode, Opcode::Nop);
        assert_eq!(instr.operands.len(), 0);
    } else {
        panic!("Expected instruction");
    }
    
    // Check inc r1 instruction
    if let FuncBody::Instruction(ref instr) = ast.instrs[1] {
        assert_eq!(instr.opcode, Opcode::Inc);
        assert_eq!(instr.operands.len(), 1);
        if let Operand::Register(reg) = instr.operands[0] {
            assert_eq!(reg, 1);
        } else {
            panic!("Expected register operand");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_parse_immediate_operands() {
    let source = "lim r2 42\nbsl 7\nbsr 15";
    let ast = parse(source, "test.s");
    
    assert_eq!(ast.instrs.len(), 3);
    
    // Check lim r2 42
    if let FuncBody::Instruction(ref instr) = ast.instrs[0] {
        assert_eq!(instr.opcode, Opcode::Lim);
        assert_eq!(instr.operands.len(), 2);
        
        if let Operand::Register(reg) = instr.operands[0] {
            assert_eq!(reg, 2);
        } else {
            panic!("Expected register operand");
        }
        
        if let Operand::Immediate(imm) = instr.operands[1] {
            assert_eq!(imm, 42);
        } else {
            panic!("Expected immediate operand");
        }
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_parse_address_operands() {
    let source = "pst $3\npld $0\npsi $1 123";
    let ast = parse(source, "test.s");
    
    assert_eq!(ast.instrs.len(), 3);
    
    // Check pst $3
    if let FuncBody::Instruction(ref instr) = ast.instrs[0] {
        assert_eq!(instr.opcode, Opcode::Pst);
        if let Operand::Address(addr) = instr.operands[0] {
            assert_eq!(addr, 3);
        } else {
            panic!("Expected address operand");
        }
    }
    
    // Check psi $1 123
    if let FuncBody::Instruction(ref instr) = ast.instrs[2] {
        assert_eq!(instr.opcode, Opcode::Psi);
        assert_eq!(instr.operands.len(), 2);
        
        if let Operand::Address(addr) = instr.operands[0] {
            assert_eq!(addr, 1);
        } else {
            panic!("Expected address operand");
        }
        
        if let Operand::Immediate(imm) = instr.operands[1] {
            assert_eq!(imm, 123);
        } else {
            panic!("Expected immediate operand");
        }
    }
}

#[test]
fn test_parse_condition_operands() {
    let source = "brc jmp .target\nbrc eq .other\nbrc neq .third";
    let ast = parse(source, "test.s");
    
    assert_eq!(ast.instrs.len(), 3);
    
    // Check brc jmp .target
    if let FuncBody::Instruction(ref instr) = ast.instrs[0] {
        assert_eq!(instr.opcode, Opcode::Brc);
        assert_eq!(instr.operands.len(), 2);
        
        if let Operand::Condition(cond) = instr.operands[0] {
            assert_eq!(cond as u8, Condition::Jmp as u8);
        } else {
            panic!("Expected condition operand");
        }
        
        if let Operand::Label(ref label) = instr.operands[1] {
            assert_eq!(label, ".target");
        } else {
            panic!("Expected label operand");
        }
    }
}

#[test]
fn test_parse_labels() {
    let source = r#"
.start
nop
.loop
inc r1
brc jmp .loop
"#;
    let ast = parse(source, "test.s");
    
    assert_eq!(ast.instrs.len(), 5);
    
    // Check .start label
    if let FuncBody::Label(ref label) = ast.instrs[0] {
        assert_eq!(label, ".start");
    } else {
        panic!("Expected label");
    }
    
    // Check .loop label
    if let FuncBody::Label(ref label) = ast.instrs[2] {
        assert_eq!(label, ".loop");
    } else {
        panic!("Expected label");
    }
}

#[test]
fn test_parse_functions() {
    let source = r#"
nop
FUNC .test_func
inc r1
dec r2
END
hlt
"#;
    let ast = parse(source, "test.s");
    
    assert_eq!(ast.instrs.len(), 2); // nop and hlt
    assert_eq!(ast.functions.len(), 1);
    
    let (func_name, func_body) = &ast.functions[0];
    assert_eq!(func_name, ".test_func");
    assert_eq!(func_body.len(), 2);
    
    // Check function body
    if let FuncBody::Instruction(ref instr) = func_body[0] {
        assert_eq!(instr.opcode, Opcode::Inc);
    } else {
        panic!("Expected instruction in function body");
    }
}

#[test]
fn test_parse_string_data() {
    let source = r#"
nop
"hello world"
hlt
"#;
    let ast = parse(source, "test.s");
    
    assert_eq!(ast.instrs.len(), 3);
    
    // Check string data
    if let FuncBody::Data(ref data) = ast.instrs[1] {
        assert_eq!(data, b"hello world");
    } else {
        panic!("Expected data");
    }
}

#[test]
fn test_parse_case_insensitive() {
    let source = "NOP\nINC R1\nDEC R2\nADD R3";
    let ast = parse(source, "test.s");
    
    assert_eq!(ast.instrs.len(), 4);
    
    // All should parse correctly regardless of case
    for instr_body in &ast.instrs {
        if let FuncBody::Instruction(_) = instr_body {
            // Successfully parsed
        } else {
            panic!("Expected instruction");
        }
    }
}

#[test]
fn test_parse_comments() {
    let source = r#"
nop // This is a comment
inc r1 // Another comment
// Full line comment
dec r2
"#;
    let ast = parse(source, "test.s");
    
    // Comments should be ignored
    assert_eq!(ast.instrs.len(), 3);
    
    // Instructions should parse normally
    if let FuncBody::Instruction(ref instr) = ast.instrs[0] {
        assert_eq!(instr.opcode, Opcode::Nop);
    }
    if let FuncBody::Instruction(ref instr) = ast.instrs[1] {
        assert_eq!(instr.opcode, Opcode::Inc);
    }
    if let FuncBody::Instruction(ref instr) = ast.instrs[2] {
        assert_eq!(instr.opcode, Opcode::Dec);
    }
}