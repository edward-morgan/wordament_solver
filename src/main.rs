use wordament_solver::*;

fn main() {
  let mut grid = Grid::init(2, 2);
  grid.set_cell('a', 1, 0, 0);
  grid.set_cell('b', 1, 0, 1);
  grid.set_cell('c', 1, 1, 0);
  grid.set_cell('d', 1, 1, 1);

  println!("{:?}", grid);
}
