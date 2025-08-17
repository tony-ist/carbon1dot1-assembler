use crate::{ast::{FuncBody, Opcode, Operand}, lower_labels::lower_labels, name_mangling::mangle, parser::parse_input_or_emit_error_and_exit, preprocessing::preprocess_defines};

pub fn assemble_func_body(func_body: &FuncBody) -> Vec<u8> {
    let mut ret = Vec::new();
    match func_body {
        FuncBody::Instruction(instr) => {
            let mut word = (instr.opcode as u8) << 3;
            let mut pushed_opword = false;
            for operand in &instr.operands {
                match operand {
                    Operand::Immediate8(a) => {
                        match instr.opcode {
                            Opcode::Bsl | Opcode::Bsr => {
                                word |= *a;
                                pushed_opword = true;
                                ret.push(word);
                            }
                            _ => {
                                if !pushed_opword {
                                    ret.push(word);
                                }
                                pushed_opword = true;
                                ret.push(*a);
                            }
                        }
                    }
                    Operand::Immediate16(a) => {
                        match instr.opcode {
                            Opcode::Brc => {
                                if *a > 0x7FFF {
                                    panic!("BRC address {:#06x} exceeds 32KiB addressable space (15-bit)", a);
                                }
                                ret.push((*a >> 7) as u8);
                                ret.push((*a & 0b01111111) as u8);
                            }
                            _ => {
                                panic!("16-bit immediate only supported for BRC instruction");
                            }
                        }
                    }
                    Operand::Condition(c) => {
                        word |= *c as u8;
                        pushed_opword = true;
                        ret.push(word);
                    }
                    Operand::Register(r) | Operand::Address(r) => {
                        word |= *r;
                        pushed_opword = true;
                        ret.push(word);
                    }
                    _ => unreachable!(),
                }
            }
            if !pushed_opword {
                ret.push(word);
            }
        }
        FuncBody::Data(d) => {
            ret.extend(d);
        }
        _ => unreachable!(),
    }
    ret
}

pub fn assemble(instrs: &Vec<FuncBody>) -> Vec<u8> {
    let mut ret = Vec::new();
    for func_body in instrs {
        ret.extend(assemble_func_body(func_body));
    }
    ret
}

pub fn assemble_source_or_emit_error_and_exit(source: &str) -> Vec<u8> {
    let preprocessed = preprocess_defines(source);
    let ast = parse_input_or_emit_error_and_exit(&preprocessed);
    let mangled = mangle(ast);
    let lowered = lower_labels(mangled);
    let assembled = assemble(&lowered);
    assembled
}
