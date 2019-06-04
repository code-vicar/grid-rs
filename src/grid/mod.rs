pub mod cell;
pub mod img;

use std::collections::HashMap;
use std::fmt;
use gust::Graph;
use rand::Rng;
use cell::*;
use img::*;

#[derive(Debug)]
pub struct Neighbors<'a> {
  pub north: Option<&'a Cell>,
  pub east: Option<&'a Cell>,
  pub south: Option<&'a Cell>,
  pub west: Option<&'a Cell>
}

#[derive(Debug)]
pub struct Grid {
  height: usize,
  width: usize,
  cells: HashMap<GridCoords, Cell>,
  graph: Graph<Cell>,
}

impl Grid {
  pub fn new(height: usize, width: usize) -> Grid {
    let mut grid = Grid {
      height,
      width,
      cells: HashMap::new(),
      graph: Graph::new(),
    };
    for col_index in 0..width {
      for row_index in 0..height {
        let coords = GridCoords {
          col_index,
          row_index
        };
        let cell = Cell::new(&coords);
        grid.cells.insert(coords, cell);
      }
    }
    grid
  }

  pub fn cells(&self) -> &HashMap<GridCoords, Cell> {
    &self.cells
  }

  pub fn cell_at(&self, coords: &GridCoords) -> Option<&Cell> {
    self.cells.get(coords)
  }

  fn _each_row(&self, reverse: bool) -> Vec<Vec<GridCoords>> {
    let mut rows = Vec::new();
    if self.height == 0 {
      return rows;
    }
    let height_max_index = self.height - 1;
    for idx in 0..self.height {
      let row_index;
      if reverse {
        row_index = height_max_index - idx;
      } else {
        row_index = idx;
      }
      let mut coords = Vec::new();
      for col_index in 0..self.width {
        coords.push(GridCoords {
          col_index,
          row_index
        });
      }
      rows.push(coords);
    }
    rows
  }

  pub fn coords(&self) -> Vec<GridCoords> {
    let mut coords = Vec::new();
    for col_index in 0..self.width {
      for row_index in 0..self.height {
        coords.push(GridCoords {
          col_index,
          row_index
        });
      }
    }
    coords
  }

  pub fn rows(&self) -> Vec<Vec<GridCoords>> {
    self._each_row(false)
  }

  pub fn rows_reverse(&self) -> Vec<Vec<GridCoords>> {
    self._each_row(true)
  }

  pub fn rand_cell(&self) -> &Cell {
     let row_index = rand::thread_rng().gen_range(0, self.width);
     let col_index = rand::thread_rng().gen_range(0, self.height);
     // we can unwrap here since the calculation is
     // bound to the size of the grid
     self.cell_at(&GridCoords {
        row_index,
        col_index
     }).unwrap()
  }

  pub fn neighbors(&self, cell: &Cell) -> Neighbors {
    Neighbors {
      north: self.north(cell),
      east: self.east(cell),
      south: self.south(cell),
      west: self.west(cell),
    }
  }

  pub fn links(&self, cell: &Cell) -> Vec<&Cell> {
    self.graph.get_adjacent(cell.coords()).into_iter().map(|coords| {
      self.cell_at(coords).unwrap()
    }).collect()
  }

  pub fn link(&mut self, source: &GridCoords, destination: &GridCoords) {
    self.graph.add_edge(source.clone(), destination.clone());
  }

  pub fn link_bidi(&mut self, source: &GridCoords, destination: &GridCoords) {
    self.graph.add_edge_bidi(source.clone(), destination.clone());
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn north(&self, cell: &Cell) -> Option<&Cell> {
    let north_coords = cell.north_coords();
    match north_coords {
      Some(coords) => {
        if coords.row_index >= self.height {
          return None
        }
        self.cell_at(&coords)
      }
      None => None
    }
  }

  pub fn east(&self, cell: &Cell) -> Option<&Cell> {
    let east_coords = cell.east_coords();
    match east_coords {
      Some(coords) => {
        if coords.col_index >= self.width {
          return None
        }
        self.cell_at(&coords)
      }
      None => None
    }
  }

  pub fn south(&self, cell: &Cell) -> Option<&Cell> {
    let south_coords = cell.south_coords();
    match south_coords {
      Some(coords) => {
        self.cell_at(&coords)
      }
      None => None
    }
  }

  pub fn west(&self, cell: &Cell) -> Option<&Cell> {
    let west_coords = cell.west_coords();
    match west_coords {
      Some(coords) => {
        self.cell_at(&coords)
      }
      None => None
    }
  }

  pub fn to_img(&self, path: &str, cell_size: u32) -> GridImage {
    let grid_image = to_img(self, cell_size);
    grid_image.canvas.save(path).unwrap();
    grid_image
  }

  pub fn to_img_with_solution(&self, path: &str, cell_size: u32, solution: &Vec<GridCoords>) -> GridImage {
    let mut grid_image = to_img(self, cell_size);
    grid_image = draw_solution(grid_image, solution);
    grid_image.canvas.save(path).unwrap();
    grid_image
  }
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let top_corner = String::from("+");
    let top_border = String::from("---+").repeat(self.width);
    let mut lines = vec![format!("{}{}", top_corner, top_border)];

    let rows = self.rows_reverse();
    for row in rows {
      let mut top = String::new();
      let mut bottom = String::new();

      for cell_id in row {
        let neighbors = self.neighbors(self.cell_at(&cell_id).unwrap());
        let links = self.links(self.cell_at(&cell_id).unwrap());

        match neighbors.west {
          None => {
            top.push_str("|");
          },
          Some(cell) => {
            if links.iter().any(|linked_cell| *linked_cell == cell) {
              top.push_str(" ");
            } else {
              top.push_str("|");
            }
          }
        }
        bottom.push_str("+");

        top.push_str("   ");
        match neighbors.south {
          None => {
            bottom.push_str("---");
          },
          Some(cell) => {
            if links.iter().any(|linked_cell| *linked_cell == cell) {
              bottom.push_str("   ");
            } else {
              bottom.push_str("---");
            }
          }
        }
      }

      top.push_str("|");
      bottom.push_str("+");
      lines.push(top);
      lines.push(bottom);
    }

    for line in lines {
      if let Result::Err(e) = writeln!(f, "{}", line) {
        panic!(e);
      }
    }

    Ok(())
  }
}
