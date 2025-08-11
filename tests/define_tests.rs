use test_case::test_case;
use carbon1dot1_assembler::assemble_source;

#[test_case("VALUE", 42 ; "simple name")]
#[test_case("MAX_COUNT", 255 ; "with underscore in the middle")]
#[test_case("_INTERNAL", 5 ; "with underscore in the beginning")]
fn test_define_variable_used_twice(var_name: &str, var_value: u8) {
    let source = format!(r#"
        @define {var_name} {var_value}
        LIM r1 {var_name}
        LIM r2 {var_name}
    "#);
    
    let result = assemble_source(&source);
    let expected = vec![0x79, var_value, 0x7A, var_value];
    assert_eq!(result, expected);
}
