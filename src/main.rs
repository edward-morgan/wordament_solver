// mod dictionary;
// mod grid;
// mod lib;

use wordament_solver::grid;
use wordament_solver::dictionary::VecDictionary;
use wordament_solver::Solver;

use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

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

  let path = Path::new("words_alpha.txt");
  let mut file = match File::open(&path) {
      Err(why) => panic!("couldn't open {}: {}", path.display(), why),
      Ok(file) => file,
  };
  let reader = BufReader::new(&file);

  let mut words: Vec<String> = reader.lines().filter(|x| x.is_ok()).map(|x| x.unwrap()).collect();
  let dbd = VecDictionary::new(&words);
  println!("Created dictionary");

  // println!("Dictionary: \n{}", dbd.to_string());

  // TODO: These should all be moved to /tests
    let source_dictionary: Vec<&str> = vec!["ad", "bad", "cab", "cad", "ab", "a"];
    // println!("{}", VecDictionary::to_string(&dbd));

    let solver = Solver::<VecDictionary>{ grid: grid, dictionary: dbd };
    println!("Solution: {}", solver.solve_grid())
}
