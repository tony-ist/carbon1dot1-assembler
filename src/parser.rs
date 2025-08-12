use codespan_reporting::{diagnostic::Diagnostic, files::SimpleFile, term::{self, termcolor::{ColorChoice, StandardStream}}};
use lalrpop_util::{lexer::Token, lalrpop_mod, ParseError};

use crate::ast::TopLvl;

lalrpop_mod!(parser);

pub fn format_parse_error(
    error: &ParseError<usize, Token<'_>, &str>,
) -> Diagnostic<()> {
    match error {
        ParseError::InvalidToken { location } => {
            Diagnostic::error().with_message("Invalid token").with_labels(vec![
                codespan_reporting::diagnostic::Label::primary((), *location..*location)
                    .with_message("Invalid token"),
            ])
        }
        ParseError::ExtraToken { token } => {
            Diagnostic::error().with_message("Extra token").with_labels(vec![
                codespan_reporting::diagnostic::Label::primary((), token.0..token.2)
                    .with_message("Extra token"),
            ])
        }
        ParseError::UnrecognizedEof { location, expected } => {
            Diagnostic::error().with_message("Unexpected end of file").with_labels(vec![
                codespan_reporting::diagnostic::Label::primary((), *location..*location)
                    .with_message(format!("Expected one of: {}", expected.join(", "))),
            ])
        }
        ParseError::UnrecognizedToken { token, expected } => {
            Diagnostic::error().with_message("Unrecognized token").with_labels(vec![
                codespan_reporting::diagnostic::Label::primary((), token.0..token.2)
                    .with_message(format!("Expected one of: {}", expected.join(", "))),
            ])
        }
        _ => unreachable!(),
    }
}

fn emit_parse_error(error: ParseError<usize, Token<'_>, &str>, text: &str) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    let diagnostic = format_parse_error(&error);
    
    term::emit(
        &mut writer.lock(),
        &config,
        &SimpleFile::new("emit_parse_error_file", text),
        &diagnostic,
    )
    .unwrap();
}

pub fn parse(input: &str) -> Result<TopLvl, ParseError<usize, Token<'_>, &str>> {
    parser::TopLvlParser::new().parse(input)
}

pub fn parse_input_or_emit_error_and_exit(input: &str) -> TopLvl {
    match parser::TopLvlParser::new().parse(input) {
        Ok(ast) => ast,
        Err(error) => {
            emit_parse_error(error, input);
            std::process::exit(1);
        }
    }
}
