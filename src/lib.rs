use std::fmt;


// Represents a solution for a Grid with a dictionary
pub struct Solution {
  words_found: Vec<String>,
  score: u32
}
impl Solution {
  pub fn new(words_found: Vec<String>, score: u32) -> Solution {
    Solution { words_found, score }
  }
}
impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "score: {}, words: ", self.score);
      for word in &self.words_found {
        write!(f, "{}, ", word);
      }
      write!(f, "")
    }
}