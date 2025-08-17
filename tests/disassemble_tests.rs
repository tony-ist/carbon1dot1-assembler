use test_case::test_case;
use carbon1dot1_assembler::{ast::{Condition, FuncBody, Instruction, Opcode, Operand}, disassemble::{disassemble, disassemble_func_body}};

#[test_case(Opcode::Nop, vec![], "nop" ; "NOP")]
#[test_case(Opcode::Inc, vec![Operand::Register(1)], "inc r1" ; "INC R1")]
#[test_case(Opcode::Dec, vec![Operand::Register(2)], "dec r2" ; "DEC R2")]
#[test_case(Opcode::Add, vec![Operand::Register(3)], "add r3" ; "ADD R3")]
#[test_case(Opcode::Adr, vec![Operand::Register(4)], "adr r4" ; "ADR R4")]
#[test_case(Opcode::Neg, vec![Operand::Register(5)], "neg r5" ; "NEG R5")]
#[test_case(Opcode::Sub, vec![Operand::Register(6)], "sub r6" ; "SUB R6")]
#[test_case(Opcode::Bsb, vec![Operand::Register(7)], "bsb r7" ; "BSB R7")]
#[test_case(Opcode::Cmp, vec![Operand::Register(1)], "cmp r1" ; "CMP R1")]
#[test_case(Opcode::Bor, vec![Operand::Register(2)], "bor r2" ; "BOR R2")]
#[test_case(Opcode::And, vec![Operand::Register(3)], "and r3" ; "AND R3")]
#[test_case(Opcode::Xor, vec![Operand::Register(4)], "xor r4" ; "XOR R4")]
#[test_case(Opcode::Bsl, vec![Operand::Immediate8(3)], "bsl 0x3" ; "BSL 3")]
#[test_case(Opcode::Bsr, vec![Operand::Immediate8(5)], "bsr 0x5" ; "BSR 5")]
#[test_case(Opcode::Rst, vec![Operand::Register(1)], "rst r1" ; "RST R1")]
#[test_case(Opcode::Rld, vec![Operand::Register(2)], "rld r2" ; "RLD R2")]
#[test_case(Opcode::Mst, vec![Operand::Register(3)], "mst r3" ; "MST R3")]
#[test_case(Opcode::Mld, vec![Operand::Register(4)], "mld r4" ; "MLD R4")]
#[test_case(Opcode::Psh, vec![], "psh" ; "PSH")]
#[test_case(Opcode::Pop, vec![], "pop" ; "POP")]
#[test_case(Opcode::Pst, vec![Operand::Address(2)], "pst $2" ; "PST $2")]
#[test_case(Opcode::Pld, vec![Operand::Address(3)], "pld $3" ; "PLD $3")]
#[test_case(Opcode::Hlt, vec![], "hlt" ; "HLT")]
#[test_case(Opcode::Fls, vec![], "fls" ; "FLS")]
#[test_case(Opcode::Lim, vec![Operand::Register(1), Operand::Immediate8(42)], "lim r1 0x2a" ; "LIM R1, 42")]
#[test_case(Opcode::Lim, vec![Operand::Register(0), Operand::Immediate8(255)], "lim r0 0xff" ; "LIM R0, 255")]
#[test_case(Opcode::Psi, vec![Operand::Address(3), Operand::Immediate8(0xFF)], "psi $3 0xff" ; "PSI $3, 255")]
#[test_case(Opcode::Brc, vec![Operand::Condition(Condition::Jmp), Operand::Immediate16(0x1234)], "brc jmp 0x1234" ; "BRC JMP 0x1234")]
#[test_case(Opcode::Brc, vec![Operand::Condition(Condition::Even), Operand::Immediate16(0x5678)], "brc even 0x5678" ; "BRC EVEN 0x5678")]
#[test_case(Opcode::Brc, vec![Operand::Condition(Condition::Eq), Operand::Immediate16(0x9ABC)], "brc eq 0x9abc" ; "BRC EQ 0x9ABC")]
#[test_case(Opcode::Brc, vec![Operand::Condition(Condition::Neq), Operand::Immediate16(0xDEF0)], "brc neq 0xdef0" ; "BRC NEQ 0xDEF0")]
#[test_case(Opcode::Brc, vec![Operand::Condition(Condition::Gt), Operand::Immediate16(0x1111)], "brc gt 0x1111" ; "BRC GT 0x1111")]
#[test_case(Opcode::Brc, vec![Operand::Condition(Condition::Lt), Operand::Immediate16(0x2222)], "brc lt 0x2222" ; "BRC LT 0x2222")]
#[test_case(Opcode::Brc, vec![Operand::Condition(Condition::Gteq), Operand::Immediate16(0x3333)], "brc gteq 0x3333" ; "BRC GTEQ 0x3333")]
#[test_case(Opcode::Brc, vec![Operand::Condition(Condition::Lteq), Operand::Immediate16(0x4444)], "brc lteq 0x4444" ; "BRC LTEQ 0x4444")]
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
    let expected = vec!["0000: lim r1 0x2a", "0002: brc jmp 0x1234", "0005: hlt"];
    assert_eq!(result, expected);
}
