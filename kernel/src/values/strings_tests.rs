


use crate::values::strings::remove_all_whitespace;


#[test]
fn remove_all_whitespace_successfully() {
    let input = "  a     b    c d e    f  ";
    let expected = "abcdef";
    let result = remove_all_whitespace(input);
    assert_eq!(result, expected);
}