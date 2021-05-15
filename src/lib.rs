use std::fmt;
use std::ops::{Add, AddAssign};

pub mod dictionary;
pub mod grid;

pub struct Solver<T: dictionary::Dictionary> {
  dictionary: T,
  grid: grid::Grid,
}

impl<T: dictionary::Dictionary> Solver<T> {

  pub fn new(dictionary: T, grid: grid::Grid) -> Self {
    Solver::<T> { dictionary, grid}
  }
  pub fn solve_grid(self) -> Solution {
    let mut final_solution: Solution = Solution::default();
    // TODO: this could probably be parallelized
    for row in 0..self.grid.height {
      for col in 0..self.grid.width {
        let start = &self.grid.get(row, col);
        let s: Solution = self.find_words_from(
          row,
          col,
          format!("{}", start.letter).as_str(),
          start.value,
          &mut vec![vec![false; self.grid.width]; self.grid.height]
        );
        final_solution += s;
      }
    }
    final_solution
  }

  /* Starting at (row, col), find all words emanating from that letter.
   */
  fn find_words_from(
    &self,
    row: usize,
    col: usize,
    word_acc: &str,
    score: u32,
    visited_cells: &mut Vec<Vec<bool>>,
  ) -> Solution {
    let mut soln = Solution::default();
    // This cell has now been visited
    visited_cells[row][col] = true;
    // First, check if the current candidate is a word
    let (is_word, is_terminal) = dictionary::Dictionary::is_word(&self.dictionary, word_acc);
    if is_word {
      soln.found(String::from(word_acc), score);
    }
    // If this word has no subsequent words, stop recursing
    if !is_terminal {
      let possible_neighbors: &[Option<grid::Cell>; 8] = &self.grid.find_neighbors(row, col);
      let row_mvmts: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
      let col_mvmts: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
      for i in 0..8 {
        match &possible_neighbors[i] {
          Some(l) => {
            // Make sure the neighbor hasn't been visited before
            let neighbor_row = (row as i32 + row_mvmts[i]) as usize;
            let neighbor_col = (col as i32 + col_mvmts[i]) as usize;
            if !visited_cells[neighbor_row][neighbor_col] {
              let w = format!("{}{}", word_acc, l.letter);
              let s = self.find_words_from(
                neighbor_row,
                neighbor_col,
                w.as_str(),
                score + l.value,
                visited_cells,
              );
              soln.add_soln(&s);
            }
          }
          None => {}
        }
      }
    }
    soln
  }
}

// Represents a solution for a Grid with a dictionary
pub struct Solution {
  words_found: Vec<String>,
  score: u32,
}
impl Solution {
  /** Form a solutuion with a list of words found, as well as a total score for the solution.
   */
  pub fn new(words_found: Vec<String>, score: u32) -> Solution {
    Solution { words_found, score }
  }

  /**
   * Add a word to this Solution
   */
  pub fn found(&mut self, word: String, score: u32) -> () {
    self.words_found.push(word);
    self.score += score;
  }

  /**
   * Add another Solution to this one. The other Solution is not consumed.
   */
  pub fn add_soln(&mut self, other: &Self) {
    let all_words =
      Vec::<String>::from([self.words_found.as_slice(), other.words_found.as_slice()].concat());
    self.words_found = all_words;
    self.score += other.score;
  }
}

impl Default for Solution {
  fn default() -> Solution {
    Solution {
      words_found: Vec::default(),
      score: 0,
    }
  }
}
impl fmt::Display for Solution {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let res: fmt::Result = write!(f, "score: {}, words: ", self.score);
    if res.is_err() {
      return res;
    }

    for word in &self.words_found {
      let res: fmt::Result = write!(f, "{}, ", word);
      if res.is_err() {
        return res;
      }
    }
    write!(f, "")
  }
}

impl Add for Solution {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    let all_words =
      Vec::<String>::from([self.words_found.as_slice(), other.words_found.as_slice()].concat());
    Self {
      words_found: all_words,
      score: self.score + other.score,
    }
  }
}

impl AddAssign for Solution {
  fn add_assign(&mut self, other: Self) {
    let all_words =
      Vec::<String>::from([self.words_found.as_slice(), other.words_found.as_slice()].concat());
    *self = Self {
      words_found: all_words,
      score: self.score + other.score,
    };
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
    assert!(
      s3.words_found
        == vec![
          String::from("one"),
          String::from("two"),
          String::from("three"),
          String::from("four")
        ]
    );
  }
}
