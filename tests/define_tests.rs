use carbon1dot1_assembler::assemble_source::assemble_source_or_emit_error_and_exit;
use test_case::test_case;

#[test_case("VALUE", 42 ; "simple name")]
#[test_case("MAX_COUNT", 255 ; "with underscore in the middle")]
#[test_case("_INTERNAL", 5 ; "with underscore in the beginning")]
fn test_define_variable_names(var_name: &str, var_value: u8) {
    let source = format!(r#"
        @define {var_name} {var_value}
        LIM r1 @{var_name}
        LIM r2 @{var_name}
        HLT
    "#);
    
    let result = assemble_source_or_emit_error_and_exit(&source);
    let expected = vec![0x79, var_value, 0x7A, var_value, 0xF0];
    assert_eq!(result, expected);
}

#[test_case("HEX_VAR", "0xFF", 255 ; "hex maximum value")]
#[test_case("HEX_SMALL", "0x0A", 10 ; "hex small value")]
#[test_case("HEX_ZERO", "0x00", 0 ; "hex zero value")]
#[test_case("DECIMAL_VAR", "123", 123 ; "decimal value")]
#[test_case("DECIMAL_MAX", "255", 255 ; "decimal maximum")]
#[test_case("DECIMAL_ZERO", "0", 0 ; "decimal zero")]
#[test_case("BINARY_VAR", "0b101010", 42 ; "binary value")]
#[test_case("BINARY_MAX", "0b11111111", 255 ; "binary maximum")]
#[test_case("BINARY_ZERO", "0b00000000", 0 ; "binary zero")]
fn test_define_different_value_formats(var_name: &str, var_literal: &str, expected_value: u8) {
    let source = format!(r#"
        @define {var_name} {var_literal}
        LIM r1 @{var_name}
    "#);
    
    let result = assemble_source_or_emit_error_and_exit(&source);
    let expected = vec![0x79, expected_value];
    assert_eq!(result, expected);
}

// TODO: Make tests more narrow by rewriting to test preprocessor.rs instead of assemble_source.rs
