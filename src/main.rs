use wordament_solver::dictionary::GraphDictionary;
use wordament_solver::grid;
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
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(&file);

    let words: Vec<String> = reader
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    let dbd = GraphDictionary::new(&words);
    println!("Instantiated dictionary");

    let solver = Solver::<GraphDictionary>::new(dbd, grid);
    println!("Solution: {}", solver.solve_grid())
}
