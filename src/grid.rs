use std::fmt;
use std::option::Option;
/* This represents the grid of letters to pull words out of. Things to think about going forward:
 * - Is a Vec<Vec<Cell>> the most efficient data structure to use here? What would be better?
 */
pub struct Grid {
  cells: Vec<Vec<Cell>>,
  width: usize,
  height: usize,
}
impl Grid {
  /** Initialize a Grid with a width/height. All cells will be set to a default value.
   */
  pub fn init(width: usize, height: usize) -> Grid {
    Grid {
      width,
      height,
      cells: vec![vec![Cell::default(); width]; height],
    }
  }

  /** Update the value in a cell.
   */
  pub fn set_cell(&mut self, letter: char, value: i32, row: usize, col: usize) {
    self.cells[row][col].letter = letter;
    self.cells[row][col].value = value;
  }

  /** Returns a slice of string slices that each represent a neighbor of the current letter.
   * [0]   [1]    [2]  
   * [3]   (r,c)  [4]
   * [5]   [6]    [7]
   */
  pub fn find_neighbors(self, prefix: &str, row: usize, col: usize) -> [Option<Cell>; 8] {
    let left_bound = col - 1 > 0;
    let upper_bound = row - 1 > 0;
    let right_bound = row + 1 < self.width;
    let bottom_bound = col + 1 < self.height;

    let possible_neighbors: [Option<Cell>; 8] = [
      if upper_bound && left_bound {Some(self.cells[row - 1][col - 1])} else {None},
      if upper_bound {Some(self.cells[row - 1][col])} else {None},
      if upper_bound && right_bound {Some(self.cells[row - 1][col + 1])} else {None},
      if left_bound {Some(self.cells[row][col - 1])} else {None},
      if right_bound {Some(self.cells[row][col + 1])} else {None},
      if bottom_bound && left_bound {Some(self.cells[row + 1][col - 1])} else {None},
      if bottom_bound {Some(self.cells[row + 1][col])} else {None},
      if bottom_bound && right_bound {Some(self.cells[row + 1][col + 1])} else {None},
    ];
    possible_neighbors
  }
}
// Pretty-printing the grid
impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      for row in &self.cells {
        let res: fmt::Result = write!(f, "| ");
        if res.is_err() { return res }
        for cell in row {
          let res: fmt::Result = write!(f, "{:?} | ", cell);
          if res.is_err() { return res }
        }
        let res: fmt::Result = write!(f, "\n");
        if res.is_err() { return res }
      }
      write!(f, "")
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
  letter: char,
  value: i32,
}
impl Default for Cell {
  // TODO: make sure this default value is okay
  fn default() -> Cell {
    Cell {
      letter: 'a',
      value: 0,
    }
  }
}
// Pretty-printing a cell
impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}: {})", self.letter, self.value)
    }
}
