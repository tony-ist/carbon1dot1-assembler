use crate::{assembler::assemble_func_body, ast::{FuncBody, Opcode, Operand}, lower_labels::lower_labels, name_mangling::mangle, parser::parse_input_or_emit_error_and_exit, preprocessing::preprocess_defines, util::format_address};

pub fn disassemble(func_bodies: &Vec<FuncBody>, include_binary: bool) -> Vec<String> {
    let mut result = Vec::new();
    let mut offset = 0u16;
    for func_body in func_bodies {
        let disassembled = disassemble_func_body(func_body);
        let assembled = assemble_func_body(func_body);

        // This is a workaround for carbon bug which leads to undefined behavior if a branch is too close to the start of a new page
        // or jumps close before a page end.
        if let FuncBody::Instruction(instr) = func_body {
            if instr.opcode == Opcode::Brc {
                let border_offset = 0x7A;
                let page_offset = offset & 0x7F;
                if page_offset >= border_offset {
                    eprintln!("Warning: Branch {} at {} is too close to new page start.", disassembled, format_address(offset));
                }
                
                let target_address: u16 = match instr.operands[1] {
                    Operand::Immediate16(a) => a,
                    _ => panic!("Expected immediate16 operand for BRC"),
                };

                let brc_page_address = offset >> 7;
                let target_page_address = target_address >> 7;
                let target_page_offset = target_address & 0x7F;

                if brc_page_address != target_page_address && target_page_offset >= border_offset {
                    eprintln!("Warning: Branch {} at {} jumps too close to new page start.", disassembled, format_address(offset));
                }
            }
        }

        if include_binary {
            let binary = assembled.iter().map(|e| format!("{:08b}", e)).collect::<Vec<String>>().join(" ");
            result.push(format!("{}: ({}) {}", format_address(offset), binary, disassembled));
        } else {
            result.push(format!("{}: {}", format_address(offset), disassembled));
        }
        offset += assembled.len() as u16;
    }
    result
}

pub fn disassemble_func_body(func_body: &FuncBody) -> String {
    format!("{}", func_body)
}

pub fn disassemble_source_or_emit_error_and_exit(source: &str, include_binary: bool) -> Vec<String> {
    let preprocessed = preprocess_defines(source);
    let ast = parse_input_or_emit_error_and_exit(&preprocessed);
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let disassembled = disassemble(&lowered, include_binary);
    disassembled
}
