use test_case::test_case;
use carbon1dot1_assembler::{ast::*, assembler::assemble};

fn create_instruction(opcode: Opcode, operands: Vec<Operand>) -> Vec<FuncBody> {
    vec![FuncBody::Instruction(Instruction { opcode, operands })]
}

#[test_case(Opcode::Nop, vec![], 0x00 ; "NOP instruction")]
#[test_case(Opcode::Inc, vec![Operand::Register(1)], 0x09 ; "INC R1")]
#[test_case(Opcode::Inc, vec![Operand::Register(7)], 0x0F ; "INC R7")]
#[test_case(Opcode::Dec, vec![Operand::Register(1)], 0x11 ; "DEC R1")]
#[test_case(Opcode::Dec, vec![Operand::Register(7)], 0x17 ; "DEC R7")]
#[test_case(Opcode::Add, vec![Operand::Register(1)], 0x19 ; "ADD R1")]
#[test_case(Opcode::Add, vec![Operand::Register(7)], 0x1F ; "ADD R7")]
#[test_case(Opcode::Adr, vec![Operand::Register(1)], 0x21 ; "ADR R1")]
#[test_case(Opcode::Adr, vec![Operand::Register(7)], 0x27 ; "ADR R7")]
#[test_case(Opcode::Neg, vec![Operand::Register(1)], 0x29 ; "NEG R1")]
#[test_case(Opcode::Neg, vec![Operand::Register(7)], 0x2F ; "NEG R7")]
#[test_case(Opcode::Sub, vec![Operand::Register(1)], 0x31 ; "SUB R1")]
#[test_case(Opcode::Sub, vec![Operand::Register(7)], 0x37 ; "SUB R7")]
#[test_case(Opcode::Bsb, vec![Operand::Register(1)], 0x39 ; "BSB R1")]
#[test_case(Opcode::Bsb, vec![Operand::Register(7)], 0x3F ; "BSB R7")]
#[test_case(Opcode::Cmp, vec![Operand::Register(1)], 0x41 ; "CMP R1")]
#[test_case(Opcode::Cmp, vec![Operand::Register(7)], 0x47 ; "CMP R7")]
#[test_case(Opcode::Bor, vec![Operand::Register(1)], 0x49 ; "BOR R1")]
#[test_case(Opcode::Bor, vec![Operand::Register(7)], 0x4F ; "BOR R7")]
#[test_case(Opcode::And, vec![Operand::Register(1)], 0x51 ; "AND R1")]
#[test_case(Opcode::And, vec![Operand::Register(7)], 0x57 ; "AND R7")]
#[test_case(Opcode::Xor, vec![Operand::Register(1)], 0x59 ; "XOR R1")]
#[test_case(Opcode::Xor, vec![Operand::Register(7)], 0x5F ; "XOR R7")]
#[test_case(Opcode::Rst, vec![Operand::Register(1)], 0x81 ; "RST R1")]
#[test_case(Opcode::Rst, vec![Operand::Register(7)], 0x87 ; "RST R7")]
#[test_case(Opcode::Rld, vec![Operand::Register(1)], 0x89 ; "RLD R1")]
#[test_case(Opcode::Rld, vec![Operand::Register(7)], 0x8F ; "RLD R7")]
#[test_case(Opcode::Mst, vec![Operand::Address(1)], 0x91 ; "MST $1")]
#[test_case(Opcode::Mst, vec![Operand::Address(7)], 0x97 ; "MST $7")]
#[test_case(Opcode::Mld, vec![Operand::Address(1)], 0x99 ; "MLD $1")]
#[test_case(Opcode::Mld, vec![Operand::Address(7)], 0x9F ; "MLD $7")]
#[test_case(Opcode::Psh, vec![], 0xC0 ; "PSH")]
#[test_case(Opcode::Pop, vec![], 0xC8 ; "POP")]
#[test_case(Opcode::Pst, vec![Operand::Address(0)], 0xD0 ; "PST $0")]
#[test_case(Opcode::Pst, vec![Operand::Address(7)], 0xD7 ; "PST $7")]
#[test_case(Opcode::Pld, vec![Operand::Address(0)], 0xE0 ; "PLD $0")]
#[test_case(Opcode::Pld, vec![Operand::Address(7)], 0xE7 ; "PLD $7")]
#[test_case(Opcode::Hlt, vec![], 0xF0 ; "HLT")]
#[test_case(Opcode::Fls, vec![], 0xF8 ; "FLS")]
fn test_single_byte_instruction_encoding(opcode: Opcode, operands: Vec<Operand>, expected: u8) {
    let instructions = create_instruction(opcode, operands);
    let result = assemble(instructions);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], expected);
}

#[test_case(Opcode::Bsl, 0, 0x60 ; "BSL 0")]
#[test_case(Opcode::Bsl, 1, 0x61 ; "BSL 1")]
#[test_case(Opcode::Bsl, 2, 0x62 ; "BSL 2")]
#[test_case(Opcode::Bsl, 3, 0x63 ; "BSL 3")]
#[test_case(Opcode::Bsl, 4, 0x64 ; "BSL 4")]
#[test_case(Opcode::Bsl, 5, 0x65 ; "BSL 5")]
#[test_case(Opcode::Bsl, 6, 0x66 ; "BSL 6")]
#[test_case(Opcode::Bsl, 7, 0x67 ; "BSL 7")]
#[test_case(Opcode::Bsr, 0, 0x68 ; "BSR 0")]
#[test_case(Opcode::Bsr, 1, 0x69 ; "BSR 1")]
#[test_case(Opcode::Bsr, 2, 0x6A ; "BSR 2")]
#[test_case(Opcode::Bsr, 3, 0x6B ; "BSR 3")]
#[test_case(Opcode::Bsr, 4, 0x6C ; "BSR 4")]
#[test_case(Opcode::Bsr, 5, 0x6D ; "BSR 5")]
#[test_case(Opcode::Bsr, 6, 0x6E ; "BSR 6")]
#[test_case(Opcode::Bsr, 7, 0x6F ; "BSR 7")]
fn test_barrel_shift_encoding(opcode: Opcode, shift_amount: u8, expected: u8) {
    let instructions = create_instruction(opcode, vec![Operand::Immediate8(shift_amount)]);
    let result = assemble(instructions);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], expected);
}

#[test_case(0, 0x42, vec![0x78, 0x42] ; "LIM R0, 0x42")]
#[test_case(1, 0x42, vec![0x79, 0x42] ; "LIM R1, 0x42")]
#[test_case(7, 0xFF, vec![0x7F, 0xFF] ; "LIM R7, 0xFF")]
#[test_case(3, 0x00, vec![0x7B, 0x00] ; "LIM R3, 0x00")]
#[test_case(0, 0x00, vec![0x78, 0x00] ; "LIM R0, 0x00")]
#[test_case(7, 0x00, vec![0x7F, 0x00] ; "LIM R7, 0x00")]
#[test_case(2, 0x55, vec![0x7A, 0x55] ; "LIM R2, 0x55")]
#[test_case(4, 0xAA, vec![0x7C, 0xAA] ; "LIM R4, 0xAA")]
fn test_lim_instruction_encoding(register: u8, immediate: u8, expected: Vec<u8>) {
    let instructions = create_instruction(
        Opcode::Lim,
        vec![Operand::Register(register), Operand::Immediate8(immediate)]
    );
    let result = assemble(instructions);
    assert_eq!(result, expected);
}

#[test_case(0, 0x42, vec![0xD8, 0x42] ; "PSI $0, 0x42")]
#[test_case(3, 0xFF, vec![0xDB, 0xFF] ; "PSI $3, 0xFF")]
#[test_case(7, 0x00, vec![0xDF, 0x00] ; "PSI $7, 0x00")]
#[test_case(1, 0x55, vec![0xD9, 0x55] ; "PSI $1, 0x55")]
#[test_case(7, 0xFF, vec![0xDF, 0xFF] ; "PSI $7, 0xFF")]
#[test_case(2, 0xAA, vec![0xDA, 0xAA] ; "PSI $2, 0xAA")]
#[test_case(4, 0x01, vec![0xDC, 0x01] ; "PSI $4, 0x01")]
#[test_case(6, 0xFE, vec![0xDE, 0xFE] ; "PSI $6, 0xFE")]
fn test_psi_instruction_encoding(port: u8, immediate: u8, expected: Vec<u8>) {
    let instructions = create_instruction(
        Opcode::Psi,
        vec![Operand::Address(port), Operand::Immediate8(immediate)]
    );
    let result = assemble(instructions);
    assert_eq!(result, expected);
}

#[test_case(Condition::Jmp, 0x00, vec![0xB0, 0x00, 0x00] ; "BRC JMP, 0x00")]
#[test_case(Condition::Even, 0x00, vec![0xB1, 0x00, 0x00] ; "BRC EVEN, 0x00")]
#[test_case(Condition::Eq, 0x00, vec![0xB2, 0x00, 0x00] ; "BRC EQ, 0x00")]
#[test_case(Condition::Neq, 0x00, vec![0xB3, 0x00, 0x00] ; "BRC NEQ, 0x00")]
#[test_case(Condition::Gt, 0x00, vec![0xB4, 0x00, 0x00] ; "BRC GT, 0x00")]
#[test_case(Condition::Lt, 0x00, vec![0xB5, 0x00, 0x00] ; "BRC LT, 0x00")]
#[test_case(Condition::Gteq, 0x00, vec![0xB6, 0x00, 0x00] ; "BRC GTEQ, 0x00")]
#[test_case(Condition::Lteq, 0x00, vec![0xB7, 0x00, 0x00] ; "BRC LTEQ, 0x00")]
#[test_case(Condition::Jmp, 0x80, vec![0xB0, 0x01, 0x00] ; "BRC JMP, 0x80")]
#[test_case(Condition::Jmp, 0xFF, vec![0xB0, 0x01, 0x7F] ; "BRC JMP, 0xFF")]
fn test_branch_instruction_encoding(condition: Condition, address: u8, expected: Vec<u8>) {
    let instructions = create_instruction(
        Opcode::Brc,
        vec![Operand::Condition(condition), Operand::Immediate16(address as u16)]
    );
    let result = assemble(instructions);
    assert_eq!(result, expected);
}