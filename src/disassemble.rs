use crate::{assembler::assemble_func_body, ast::FuncBody, lower_labels::lower_labels, name_mangling::mangle, parser::parse_input_or_emit_error_and_exit, preprocessing::preprocess_defines, util::format_address};

pub fn disassemble(func_bodies: &Vec<FuncBody>, include_binary: bool) -> Vec<String> {
    let mut result = Vec::new();
    let mut offset = 0u16;
    for func_body in func_bodies {
        let disassembled = disassemble_func_body(func_body);
        let assembled = assemble_func_body(func_body);
        // todo: support programs up to 32KiB
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
