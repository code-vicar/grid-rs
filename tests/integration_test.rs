extern crate grid;
use grid::prelude::*;

fn make_grid() -> Grid {
  let grid = Grid::new(10, 10);
  grid
}

#[test]
fn graph_len() {
  let grid = make_grid();
  assert_eq!(10, grid.height());
}

#[test]
fn iterate_cells() {
  let grid = make_grid();
  let mut counter = 0;
  grid.each_cell(|_| {
    counter = counter + 1;
  });
  assert_eq!(100, counter);
}

#[test]
fn get_cell_at() {
  let grid = make_grid();
  let some_cell = grid.cell_at(1, 1);
  assert_eq!("1_1", some_cell.unwrap().get_id());
  let none_cell = grid.cell_at(10, 1);
  assert!(none_cell.is_none());
}
