use std::fmt;
use std::ops::Add;

pub mod dictionary;
pub mod grid;

// Represents a solution for a Grid with a dictionary
pub struct Solution {
  words_found: Vec<String>,
  score: u32
}
impl Solution {
  /** Form a solutuion with a list of words found, as well as a total score for the solution.
   */
  pub fn new(words_found: Vec<String>, score: u32) -> Solution {
    Solution { words_found, score }
  }

  pub fn found(mut self, word: String, score: u32) -> () {
    self.words_found.push(word);
    self.score += score;
  }
}
impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let res: fmt::Result = write!(f, "score: {}, words: ", self.score);
      if res.is_err() { return res }

      for word in &self.words_found {
        let res: fmt::Result = write!(f, "{}, ", word);
        if res.is_err() { return res }
      }
      write!(f, "")
    }
}

impl Add for Solution {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    let all_words = Vec::<String>::from([self.words_found.as_slice(), other.words_found.as_slice()].concat());
    Self {
      words_found: all_words,
      score: self.score + other.score
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  fn test_add() {
    let s1 = Solution::new(vec![String::from("one"), String::from("two")], 8);
    let s2 = Solution::new(vec![String::from("three"), String::from("four")], 10);
    let s3 = s1 + s2;
    assert!(s3.score == 18);
    assert!(s3.words_found == 
      vec![String::from("one"), String::from("two"), String::from("three"), String::from("four")]);
  }
}