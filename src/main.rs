// mod dictionary;
// mod grid;
// mod lib;

use wordament_solver::grid;
use wordament_solver::dictionary::{Dictionary, DebugDictionary};
use wordament_solver::Solution;

/** Assumptions:
 * - Words must be made by connecting letters adjacent to each other, along a path.
 * - Letters cannot be reused.
 *
 *        start
 * | (a) | (b) | --> b, a, d
 * |  c  | (d) |
 */
fn main() {
  let mut grid = grid::Grid::init(2, 2);
  grid.set('a', 1, 0, 0);
  grid.set('b', 1, 0, 1);
  grid.set('c', 1, 1, 0);
  grid.set('d', 1, 1, 1);

  println!("Grid:\n{:?}", grid);

  // Set up the dictionary
  let dbd = &DebugDictionary::new();

  // println!("Dictionary: \n{}", dbd.to_string());

  // TODO: These should all be moved to /tests
    let source_dictionary: Vec<&str> = vec!["ad", "bad", "cab", "cad", "ab", "a"];
    for word in source_dictionary {
      println!("{}: {}", word, DebugDictionary::find_word(dbd, word));
    }
    println!("{}", DebugDictionary::to_string(dbd));

  // println!("{}", find_all_words(&grid, &dictionary::DebugDictionary::new()));
}
