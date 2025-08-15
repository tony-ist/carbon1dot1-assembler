use crate::{assembler::assemble, lower_labels::lower_labels, name_mangling::mangle, parser::parse_input_or_emit_error_and_exit, preprocessing::preprocess_defines};

pub fn assemble_source_or_emit_error_and_exit<'a>(source: &'a str) -> Vec<u8> {
    let preprocessed = preprocess_defines(source);
    let ast = parse_input_or_emit_error_and_exit(&preprocessed);
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    assemble(lowered)
}
