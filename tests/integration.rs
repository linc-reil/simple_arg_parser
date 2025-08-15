use simple_arg_parser::*;

#[test]
fn test_argument_parser_get_struct() {
    let args = String::from("test.txt -o test.o --debug --ignore-warnings=true")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let test = parse_arguments(args);
    dbg!(test);
}
