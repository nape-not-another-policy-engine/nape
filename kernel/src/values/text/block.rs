#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Block {
    value: String
}

impl Block {

    /// Create a new text block
    ///
    /// A Block is multi-line string which can contain an unbounded amount of text.
    /// When a new Line is created, it is sanitized by removing all whitespace from
    /// the start and end of the input. No other formatting characters are removed.
    ///
    /// # Arguments
    ///
    /// * `s`: an &str.
    ///
    /// returns: Line
    ///
    /// # Examples
    ///
    ///
    /// use crate::values::block::Block;
    ///
    /// let text_block = Block::new("Another block of text.\r\n\t");
    ///
    pub fn new(value: &str) -> Self {
        Block { value: value.to_string() }.sanitize()
    }

    /// Create a new text block
    ///
    /// A Block is multi-line string which can contain an unbounded amount of text.
    /// When a new Line is created, it is sanitized by removing all whitespace from
    /// the start and end of the input. No other formatting characters are removed.
    ///
    /// # Arguments
    ///
    /// * `s`: a String.
    ///
    /// returns: Line
    ///
    /// # Examples
    ///
    ///
    /// use crate::values::block::Block;
    ///
    /// let text_block = Block::from_string(String::from("Another block of text.\r\n\t"));
    ///
    pub fn from_string<S: Into<String>>(s: S) -> Self {
        Block { value: s.into() }.sanitize()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    fn sanitize(&self) -> Self {
        Block { value: String::from(self.value.trim()) }
    }
}

// Implement PartialEq to compare Block to String.
impl PartialEq<String> for Block {
    fn eq(&self, other: &String) -> bool {
        self.value == *other
    }
}

// This allows for the reverse comparison: String to Block
impl PartialEq<Block> for String {
    fn eq(&self, other: &Block) -> bool {
        *self == other.value
    }
}

// Implement PartialEq to compare Block to &str.
impl PartialEq<&str> for Block {
    fn eq(&self, other: &&str) -> bool {
        self.value == *other
    }
}

// This allows for the reverse comparison: &str to Block
impl PartialEq<Block> for &str {
    fn eq(&self, other: &Block) -> bool {
        *self == other.value
    }
}

