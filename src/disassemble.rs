use crate::{assembler::{assemble_func_body}, ast::FuncBody, lower_labels::lower_labels, name_mangling::mangle, parser::parse_input_or_emit_error_and_exit, preprocessing::preprocess_defines};

pub fn disassemble(func_bodies: &Vec<FuncBody>) -> Vec<String> {
    let mut result = Vec::new();
    let mut offset = 0;
    for func_body in func_bodies {
        let disassembled = disassemble_func_body(func_body);
        // todo: support programs up to 32KiB
        result.push(format!("{:04X}: {}", offset, disassembled));
        offset += assemble_func_body(func_body).len();
    }
    result
}

pub fn disassemble_func_body(func_body: &FuncBody) -> String {
    format!("{}", func_body)
}

pub fn disassemble_source_or_emit_error_and_exit(source: &str) -> Vec<String> {
    let preprocessed = preprocess_defines(source);
    let ast = parse_input_or_emit_error_and_exit(&preprocessed);
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let disassembled = disassemble(&lowered);
    disassembled
}
