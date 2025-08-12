use lalrpop_util::{lexer::Token, ParseError};

use crate::{assembler::assemble, lower_labels::lower_labels, name_mangling::mangle, parser::{parse, parse_input_or_emit_error_and_exit}};

pub fn assemble_source<'a>(source: &'a str) -> Result<Vec<u8>, ParseError<usize, Token<'a>, &'a str>> {
    let ast = parse(source)?;
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    Ok(assemble(lowered))
}

pub fn assemble_source_or_emit_error_and_exit<'a>(source: &'a str) -> Vec<u8> {
    let ast = parse_input_or_emit_error_and_exit(source);
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    assemble(lowered)
}
