mod grid;
mod dictionary;
mod lib;

use grid::*;
use lib::Solution;

/** Assumptions:
 * - Words must be made by connecting letters adjacent to each other, along a path.
 * - Letters cannot be reused.
 * 
 *        start
 * | (a) | (b) | --> b, a, d
 * |  c  | (d) |
 */
fn main() {
  let mut grid = Grid::init(2, 2);
  grid.set_cell('a', 1, 0, 0);
  grid.set_cell('b', 1, 0, 1);
  grid.set_cell('c', 1, 1, 0);
  grid.set_cell('d', 1, 1, 1);

  println!("Grid:\n{:?}", grid);

  // Set up the dictionary
  let dbd = dictionary::DebugDictionary::new();

  println!("Dictionary: \n{}", dbd.to_string());
  // println!("{}", find_all_words(&grid, &dictionary::DebugDictionary::new()));
}

fn find_all_words(grid: &Grid, dict: &dictionary::DebugDictionary) -> Solution {
  Solution::new(vec![String::from("")], 0)
}
