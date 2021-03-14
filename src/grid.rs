use std::fmt;

/* This represents the grid of letters to pull words out of. Things to think about going forward:
 * - Is a Vec<Vec<Cell>> the most efficient data structure to use here? What would be better?
 */
pub struct Grid {
  cells: Vec<Vec<Cell>>,
  width: usize,
  height: usize,
}
impl Grid {
  pub fn init(width: usize, height: usize) -> Grid {
    Grid {
      width,
      height,
      cells: vec![vec![Cell::default(); width]; height],
    }
  }

  pub fn set_cell(&mut self, letter: char, value: i32, row: usize, col: usize) {
    self.cells[row][col].letter = letter;
    self.cells[row][col].value = value;
  }
}
impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      for row in &self.cells {
        write!(f, "| ");
        for cell in row {
          write!(f, "{:?} | ", cell);
        }
        write!(f, "\n");
      }
      write!(f, "")
    }
}

#[derive(Clone)]
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
impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}: {})", self.letter, self.value)
    }
}
