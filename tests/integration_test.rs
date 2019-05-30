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
  assert_eq!(100, grid.cells().len());
}

#[test]
fn get_cell_at() {
  let grid = make_grid();
  let coords = GridCoords {
    col_index: 0,
    row_index: 0
  };
  let some_cell = grid.cell_at(&coords);
  assert_eq!(&coords, some_cell.unwrap().coords());
  let invalid_coords = GridCoords {
    col_index: 10,
    row_index: 1
  };
  let none_cell = grid.cell_at(&invalid_coords);
  assert!(none_cell.is_none());
}

#[test]
fn get_links() {
  let mut grid = make_grid();
  let id = GridCoords {
    col_index: 0,
    row_index: 0
  };
  // do mutable borrows
  let cell = grid.cell_at(&id).unwrap();
  let north = grid.north(&cell).unwrap().coords().clone();
  grid.link_bidi(&id, &north);
  // switch to non mutable borrow
  // let grid = grid;
  let cell = grid.cell_at(&id).unwrap();
  let edges = grid.links(cell);
  assert_eq!(1, edges.len());
  assert_eq!(edges[0].coords(), &north);
}

#[test]
fn get_neighbors() {
  let grid = make_grid();
  let id = GridCoords {
    col_index: 0,
    row_index: 0
  };
  let cell = grid.cell_at(&id).unwrap();
  let neighbors = grid.neighbors(cell);
  assert!(neighbors.south.is_none());
  assert!(neighbors.west.is_none());
  assert!(neighbors.north.is_some());
  assert!(neighbors.east.is_some());
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

  for row in grid.rows() {
    assert_eq!(grid.width(), row.len());
    assert_eq!(0, row[0].col_index);
  };
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
fn binary_tree_maze_to_image_test() {
  let grid = make_grid();

  let grid = binarytree::apply_to(grid);
  println!("{}", grid);
  grid.to_img("test-output/test.png", 5);
}

#[ignore]
#[test]
fn sidewinder_tree_maze() {
  let grid = make_grid();

  let grid = sidewinder::apply_to(grid);

  println!("{}", grid);
}

#[ignore]
#[test]
fn to_image_test() {
  let grid = make_grid();

  let grid = sidewinder::apply_to(grid);
  println!("{}", grid);
  grid.to_img("test-output/test.png", 5);
}

#[ignore]
#[test]
fn shortest_path() {
  let grid = make_grid();

  let grid = sidewinder::apply_to(grid);
  let source = GridCoords {
    row_index: 0,
    col_index: 0,
  };
  let destination = GridCoords {
    row_index: 1,
    col_index: 1,
  };
  let distances = solutions::dijkstra::Dijkstra::new(&grid, &source);
  println!("Done calculating distances");
  println!("{}", grid);
  println!("{:#?}", distances.shortest_path_to(&grid, &destination))
}

#[test]
fn to_image_with_solution_test() {
  let grid = make_grid();

  let grid = sidewinder::apply_to(grid);
  println!("{}", grid);
  // grid.to_img("test-output/test.png", 5);
  let distances = solutions::dijkstra::Dijkstra::new(&grid, &GridCoords {
    col_index: 0,
    row_index: 0,
  });
  let solution = distances.shortest_path_to(&grid, &GridCoords {
    col_index: 2,
    row_index: 2,
  });
  grid.to_img_with_solution("test-output/solution.png", 6, &solution);
}
