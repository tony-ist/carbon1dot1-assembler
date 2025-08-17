use crate::{ast::FuncBody, lower_labels::lower_labels, name_mangling::mangle, parser::parse_input_or_emit_error_and_exit, preprocessing::preprocess_defines};

pub fn disassemble(func_bodies: &Vec<FuncBody>) -> Vec<String> {
    vec![]
}

pub fn disassemble_func_body(func_body: &FuncBody) -> String {
    String::new()
}

pub fn disassemble_source_or_emit_error_and_exit(source: &str) -> Vec<String> {
    let preprocessed = preprocess_defines(source);
    let ast = parse_input_or_emit_error_and_exit(&preprocessed);
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let disassembled = disassemble(&lowered);
    disassembled
}
