use std::{io::Write};

use carbon1dot1_assembler::{assembler::assemble_source_or_emit_error_and_exit, disassemble::disassemble_source_or_emit_error_and_exit, romgen};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap()]
    input_file: String,
    #[clap(short, long, default_value_t=String::from("./out.bin"))]
    output_file: String,
    #[clap(short, long, required = false, help="Also write disassembled code in disassembly file")]
    disasm_file: Option<String>,
    #[clap(short, long, required = false, default_value_t=false, help="Include binary in disassembly file")]
    include_bin: bool,
    #[clap(short, long, required = false, help="Write schem file (broken at the moment)")]
    schem_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let text = std::fs::read_to_string(&args.input_file).unwrap();
    let assembled = assemble_source_or_emit_error_and_exit(&text);
    let mut f = std::fs::File::create(&args.output_file).unwrap();
    f.write_all(
        &assembled
            .iter()
            .map(|e| format!("{:08b}", e))
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();
    
    if let Some(disasm_file) = args.disasm_file {
        let mut f = std::fs::File::create(&disasm_file).unwrap();
        let disassembly = disassemble_source_or_emit_error_and_exit(&text, args.include_bin);
        f.write_all(disassembly.join("\n").as_bytes()).unwrap();
    }
    
    if let Some(schem_file) = args.schem_file {
        let mut f = std::fs::File::create(&schem_file).unwrap();
        romgen::generate_schem(&mut f, &assembled, 256).unwrap();
    }
}
