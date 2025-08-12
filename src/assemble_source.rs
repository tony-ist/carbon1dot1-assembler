use lalrpop_util::{lexer::Token, ParseError};

use crate::{assembler::assemble, lower_labels::lower_labels, name_mangling::mangle, parser::parse};

pub fn assemble_source(source: &str) -> Result<Vec<u8>, ParseError<usize, Token<'_>, &str>> {
    let ast = parse(source)?;
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    Ok(assemble(lowered))
}