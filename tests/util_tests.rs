use test_case::test_case;
use carbon1dot1_assembler::util;

#[test_case(0x0000, "00")]
#[test_case(0x007F, "00")]
#[test_case(0x7FFF, "FF")]
#[test_case(0x4040, "80")]
fn test_format_upper_pc(pc: u16, expected: &str) {
    let result = util::format_upper_pc(pc);
    assert_eq!(result, expected);
}

#[test]
#[should_panic(expected = "pc address is larger than 15 bits (0x7FFF)")]
fn test_format_upper_pc_panic() {
    util::format_upper_pc(0x8000);
}

#[test_case(0x0000, "00")]
#[test_case(0x007F, "7F")]
#[test_case(0x7FFF, "7F")]
#[test_case(0x4040, "40")]
fn test_format_lower_pc(pc: u16, expected: &str) {
    let result = util::format_lower_pc(pc);
    assert_eq!(result, expected);
}

#[test]
#[should_panic(expected = "pc address is larger than 15 bits (0x7FFF)")]
fn test_format_lower_pc_panic() {
    util::format_lower_pc(0x8000);
}
