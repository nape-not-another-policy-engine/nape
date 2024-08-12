
use crate::values::text::line::Line;

#[test]
fn create_new_block_from_default() {
    assert_eq!(Line::default(), "");
}

#[test]
fn create_new_line_from_str() {
    assert_eq!(Line::new("This is a new line of text"),
               ("This is a new line of text"));
}

#[test]
fn create_new_line_from_string() {
    assert_eq!(Line::from_string(String::from("This is a new line of text")),
               String::from("This is a new line of text"));
}

#[test]
fn sanitize_line_when_created_from_string() {
    assert_eq!(Line::from_string(String::from("  Unsanitary \nText Line\r\t  ")),
               String::from("Unsanitary Text Line"));
}

#[test]
fn sanitize_line_when_created_from_str() {
    assert_eq!(Line::new("  Unsanitary \nText Line\r\t  "),
               ("Unsanitary Text Line"));
}

#[test]
fn check_length_of_line() {
    assert_eq!(Line::from_string("This is a new line of text").len(),
               26);
}

#[test]
fn check_line_value() {
    assert_eq!(Line::new("hello line").value(), "hello line")
}

#[test]
fn compare_line_to_string() {
    assert_eq!(Line::from_string("This is a new line of text"),
               String::from("This is a new line of text"))
}

#[test]
fn compare_string_to_line() {
    assert_eq!(String::from("This is a new line of text"),
               Line::from_string("This is a new line of text"))
}

#[test]
fn compare_line_to_str() {
    assert_eq!(Line::from_string("This is a new line of text"),
               "This is a new line of text")
}

#[test]
fn compare_str_to_line() {
    assert_eq!("This is a new line of text",
               Line::from_string("This is a new line of text"))
}

#[test]
fn compare_two_same_lines() {
    assert_eq!(Line::from_string("This is a new line of text"),
               Line::from_string("This is a new line of text"))
}

#[test]
fn compare_two_different_line() {
    assert_ne!(Line::from_string("First line of text"),
               Line::from_string("Second line of text"))
}
