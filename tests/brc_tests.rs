use test_case::test_case;
use carbon1dot1_assembler::{ast::*, assembler::assemble};

fn create_brc_jmp_instruction(address: u16) -> Vec<FuncBody> {
    vec![FuncBody::Instruction(Instruction {
        opcode: Opcode::Brc,
        operands: vec![
            Operand::Condition(Condition::Jmp),
            Operand::Immediate(address)
        ]
    })]
}

#[test_case(0x0000, vec![0xB0, 0x00, 0x00] ; "BRC JMP to address 0x0000 (start of ROM)")]
#[test_case(0x007F, vec![0xB0, 0x00, 0x7F] ; "BRC JMP to address 0x007F (last in page 0)")]
#[test_case(0x0080, vec![0xB0, 0x01, 0x00] ; "BRC JMP to address 0x0080 (first in page 1)")]
#[test_case(0x00FF, vec![0xB0, 0x01, 0x7F] ; "BRC JMP to address 0x00FF (last in page 1)")]
#[test_case(0x0100, vec![0xB0, 0x02, 0x00] ; "BRC JMP to address 0x0100 (first in page 2)")]
#[test_case(0x7FFF, vec![0xB0, 0xFF, 0x7F] ; "BRC JMP to address 0x7FFF (last valid address)")]
#[test_case(0x4040, vec![0xB0, 0x80, 0x40] ; "BRC JMP to address 0x4040 (mid-range test)")]
fn test_brc_valid_15bit_addresses(address: u16, expected: Vec<u8>) {
    let instructions = create_brc_jmp_instruction(address);
    let result = assemble(instructions);
    assert_eq!(result, expected);
}

#[test_case(0x8000 ; "BRC JMP to 0x8000 (first invalid - bit 15 set)")]
#[test_case(0xFF80 ; "BRC JMP to 0xFF80 (offset MSB set)")]
#[test_case(0x80FF ; "BRC JMP to 0x80FF (page MSB set, offset MSB set)")]
#[test_case(0xFFFF ; "BRC JMP to 0xFFFF (maximum invalid address)")]
#[test_case(0x7F80 ; "BRC JMP to 0x7F80 (valid page, invalid offset)")]
fn test_brc_invalid_addresses_should_error(address: u16) {
    let instructions = create_brc_jmp_instruction(address);
    
    let result = std::panic::catch_unwind(|| {
        assemble(instructions)
    });
    assert!(result.is_err(), "Expected error for invalid address {:#06x} but assembly succeeded", address);
}