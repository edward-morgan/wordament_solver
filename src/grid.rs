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
  pub fn set(&mut self, letter: char, value: i32, row: usize, col: usize) {
    self.cells[row][col].letter = letter;
    self.cells[row][col].value = value;
  }

  pub fn get(&self, row: usize, col: usize) -> Cell {
    self.cells[row][col]
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
  fn default() -> Cell {
    Cell {
      letter: ' ',
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


// TESTS
#[cfg(test)]
mod tests {
  use super::*;
    #[test]
    fn test_init() {
        let g1 = Grid::init(5, 4);
        assert_eq!(g1.width, 5);
        assert_eq!(g1.height, 4);
        let g2 = Grid::init(0, 0);
        assert_eq!(g2.width, 0);
        assert_eq!(g2.height, 0);
    }

    #[test]
    fn test_set_cell() {
        let mut g1 = Grid::init(5, 4);
        assert_eq!(g1.width, 5);
        assert_eq!(g1.height, 4);
        g1.set('a', 10, 0, 0);
        g1.set('b', 11, 0, 1);
        g1.set('c', 12, 0, 2);
        g1.set('d', 13, 0, 3);
        assert!(g1.get(0, 0).letter == 'a');
        assert!(g1.get(0, 0).value == 10);
        assert!(g1.get(0, 2).letter == 'c');
        assert!(g1.get(0, 2).value == 12);
    }

    #[test]
    #[should_panic]
    fn test_bad_cell() {
        let mut g1 = Grid::init(5, 4);
        assert_eq!(g1.width, 5);
        assert_eq!(g1.height, 4);
        g1.set('a', 10, 0, 0);
        g1.set('b', 11, 0, 1);
        g1.set('c', 12, 0, 2);
        g1.set('d', 13, 0, 3);
        g1.get(3,10);
    }
}