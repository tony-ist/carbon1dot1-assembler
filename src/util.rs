pub fn format_address(pc: u16) -> String {
    format!("{} {}", format_upper_pc(pc), format_lower_pc(pc))
}

pub fn format_upper_pc(pc: u16) -> String {
    if pc > 0x7FFF {
        panic!("pc address is larger than 15 bits (0x7FFF)");
    }

    format!("{:02X}", pc >> 7)
}

pub fn format_lower_pc(pc: u16) -> String {
    if pc > 0x7FFF {
        panic!("pc address is larger than 15 bits (0x7FFF)");
    }

    format!("{:02X}", pc & 0x7F)
}
