use std::{io::Write};

use carbon1dot1_assembler::{
    assemble_source::assemble_source_or_emit_error_and_exit,
};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap()]
    input_file: String,
    #[clap(short, long, default_value_t=String::from("./out.bin"))]
    output_file: String,
}

fn main() {
    let args = Args::parse();
    let text = std::fs::read_to_string(&args.input_file).expect("failed to read the input file");
    let assembled = assemble_source_or_emit_error_and_exit(&text);
    let mut f = std::fs::File::create(&args.output_file).expect("Failed to open the output file.");
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
