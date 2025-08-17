use test_case::test_case;
use carbon1dot1_assembler::{ast::{Condition, FuncBody, Instruction, Opcode, Operand}, disassemble::{disassemble, disassemble_func_body}};

#[test_case(Opcode::Lim, vec![Operand::Register(1), Operand::Immediate8(42)], "lim r1 42" ; "LIM R1, 42")]
#[test_case(Opcode::Brc, vec![Operand::Condition(Condition::Jmp), Operand::Immediate16(0x1234)], "brc jmp 0x1234" ; "BRC JMP 0x1234")]
fn test_disassemble_func_body(opcode: Opcode, operands: Vec<Operand>, expected: &str) {
    let func_body = FuncBody::Instruction(Instruction { opcode, operands });
    let result = disassemble_func_body(&func_body);
    assert_eq!(result, expected);
}

#[test]
fn test_disassemble_multiline() {
    let func_bodies = vec![
        FuncBody::Instruction(Instruction {
            opcode: Opcode::Lim,
            operands: vec![Operand::Register(1), Operand::Immediate8(42)]
        }),
        FuncBody::Instruction(Instruction {
            opcode: Opcode::Brc,
            operands: vec![Operand::Condition(Condition::Jmp), Operand::Immediate16(0x1234)]
        }),
        FuncBody::Instruction(Instruction {
            opcode: Opcode::Hlt,
            operands: vec![]
        })
    ];
    let result = disassemble(&func_bodies);
    let expected = vec!["0000: lim r1 42", "0002: brc jmp 0x1234", "0005: hlt"];
    assert_eq!(result, expected);
}
