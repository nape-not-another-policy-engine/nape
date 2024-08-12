
use crate::values::text::block::Block;

#[test]
fn create_new_block_from_default() {
    assert_eq!(Block::default(), "");
}

#[test]
fn create_new_block_from_str() {
    assert_eq!(Block::new("This is a new block of text."),
               "This is a new block of text.");
}

#[test]
fn create_new_block_from_string() {
    assert_eq!(Block::from_string(String::from("This is a new block of text.")),
               "This is a new block of text.");
}

#[test]
fn sanitize_block_when_created_from_str() {
    assert_eq!(Block::new("  Unsanitary \nText block\r\t  "),
               "Unsanitary \nText block");
}

#[test]
fn sanitize_block_when_created_from_string() {
    assert_eq!(Block::from_string(String::from("  Unsanitary \nText block\r\t  ")),
               String::from("Unsanitary \nText block"));
}

#[test]
fn check_length_of_block() {
    assert_eq!(Block::from_string("\rThis is a new \nblock of \ttext").len(),
               29);
}

#[test]
fn check_block_value() {
    assert_eq!(Block::new("hello line").value(), "hello line")
}

#[test]
fn compare_line_to_string() {
    assert_eq!(Block::from_string("This is a new block of text"),
               String::from("This is a new block of text"))
}

#[test]
fn compare_string_to_line() {
    assert_eq!(String::from("This is a new block of text"),
               Block::from_string("This is a new block of text"))
}

#[test]
fn compare_line_to_str() {
    assert_eq!(Block::from_string("This is a new block of text"),
               "This is a new block of text")
}

#[test]
fn compare_str_to_line() {
    assert_eq!("This is a new block of text",
               Block::from_string("This is a new block of text"))
}

#[test]
fn compare_two_same_blocks() {
    assert_eq!(Block::from_string("\rThis is a new \nblock of \ttext"),
               Block::from_string("\rThis is a new \nblock of \ttext"))
}

#[test]
fn compare_two_different_block() {
    assert_ne!(Block::from_string("\rThis is the first \nblock of \ttext"),
               Block::from_string("\rThis is the second \nblock of \ttext"))
}
