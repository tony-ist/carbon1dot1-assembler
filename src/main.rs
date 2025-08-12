use std::{io::Write, process::exit};

use carbon1dot1_assembler::{
    assemble_source::assemble_source, assembler::assemble, ast::TopLvl, lower_labels::lower_labels, name_mangling::mangle, parser::{format_parse_error, parse}
};
use clap::Parser;
use codespan_reporting::{
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use lalrpop_util::{lexer::Token, ParseError};

#[derive(Parser)]
struct Args {
    #[clap()]
    input_file: String,
    #[clap(short, long, default_value_t=String::from("./out.bin"))]
    output_file: String,
}

fn emit_parse_error(error: ParseError<usize, Token<'_>, &str>, filename: &str, text: &str) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    let diagnostic = format_parse_error(&error);
    
    term::emit(
        &mut writer.lock(),
        &config,
        &SimpleFile::new(filename, text),
        &diagnostic,
    )
    .unwrap();
}

fn assemble_file(input_file: &str, output_file: &str) -> Result<(), ParseError<usize, Token<'_>, &str>> {
    let text = std::fs::read_to_string(input_file).expect("failed to read the input file");
    let assembled = assemble_source(&text)?;
    let mut f = std::fs::File::create(output_file).expect("Failed to open the output file.");
    f.write_all(
        &assembled
            .iter()
            .map(|e| format!("{:08b}", e))
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();
    //romgen::generate_schem(&mut f, &assembled, 256).unwrap();
}

fn main() {
    let args = Args::parse();

    match assemble_file(&args.input_file, &args.output_file) {
        Ok(_) => (),
        Err(e) => {
            emit_parse_error(e, &args.input_file, &text);
            exit(1);
        }
    }
}
