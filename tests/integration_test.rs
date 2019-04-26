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
  let some_cell = grid.cell_at(GridCoords {
    col_index: 0,
    row_index: 0
  });
  assert_eq!("0_0", some_cell.unwrap().get_id());
  let none_cell = grid.cell_at(GridCoords {
    col_index: 10,
    row_index: 1
  });
  assert!(none_cell.is_none());
}

#[test]
fn get_links() {
  let mut grid = make_grid();
  let pos = GridCoords {
    col_index: 0,
    row_index: 0
  };
  grid.link_bidi(pos, grid.north(pos).unwrap());
  let edges = grid.links(pos);
  assert_eq!(1, edges.len());
  assert_eq!("0_0", edges[0].from);
  assert_eq!("1_0", edges[0].to);
}

#[test]
fn get_neighbors() {
  let grid = make_grid();
  let pos = GridCoords {
    col_index: 0,
    row_index: 0
  };
  let neighbors = grid.neighbors(pos);
  assert!(neighbors.south.is_boundary());
  assert!(neighbors.west.is_boundary());
  assert!(neighbors.north.is_cell());
  assert!(neighbors.east.is_cell());
}

#[ignore]
#[test]
fn get_random() {
  let grid = make_grid();
  let cell = grid.rand_cell();
  println!("{:#?}", cell)
}

#[test]
fn each_row() {
  let grid = make_grid();

  grid.each_row(|row, _| {
    let (_, loc) = row[0];
    assert_eq!(grid.width(), row.len());
    assert_eq!(0, loc.col_index);
  });
}

#[ignore]
#[test]
fn binary_tree_maze() {
  let grid = make_grid();

  let grid = binarytree::apply_to(grid);

  println!("{}", grid);
}

#[ignore]
#[test]
fn sidewinder_tree_maze() {
  let grid = make_grid();

  let grid = sidewinder::apply_to(grid);

  println!("{}", grid);
}
