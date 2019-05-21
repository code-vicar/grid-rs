mod cell;

use std::fmt;
use std::convert::TryFrom;
use gust::{Graph, GraphBuilder, Edge};
use line_rs::*;
use rand::Rng;
pub use cell::*;

#[derive(Debug)]
pub struct EdgeData {}

#[derive(Debug, Clone, Copy)]
pub enum GridPosition {
  InBounds(GridCoords),
  OutOfBounds
}

impl GridPosition {
  pub fn unwrap(&self) -> GridCoords {
    match self {
      GridPosition::InBounds(coords) => coords.to_owned(),
      _ => panic!()
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridCoords {
  pub col_index: usize,
  pub row_index: usize
}

#[derive(Debug)]
pub enum Neighbor<'a> {
  NeighborCell((GridCoords, &'a Cell)),
  GridBoundary
}

impl<'a> Neighbor<'a> {
  pub fn is_cell(&self) -> bool {
    match self {
      Neighbor::NeighborCell(_) => true,
      _ => false
    }
  }

  pub fn is_boundary(&self) -> bool {
    match self {
      Neighbor::GridBoundary => true,
      _ => false
    }
  }
}

#[derive(Debug)]
pub struct Neighbors<'a> {
  pub north: Neighbor<'a>,
  pub east: Neighbor<'a>,
  pub south: Neighbor<'a>,
  pub west: Neighbor<'a>
}

impl<'a> Neighbors<'a> {
  pub fn new() -> Neighbors<'a> {
    Neighbors {
      north: Neighbor::GridBoundary,
      east: Neighbor::GridBoundary,
      south: Neighbor::GridBoundary,
      west: Neighbor::GridBoundary
    }
  }

  pub fn set(&mut self, dir: &str, n: Neighbor<'a>) {
    match dir {
      "north" => self.north = n,
      "east" => self.east = n,
      "south" => self.south = n,
      "west" => self.west = n,
      _ => ()
    }
  }
}

#[derive(Debug)]
pub struct Grid {
  height: usize,
  width: usize,
  graph: Graph<Cell, EdgeData>,
}

impl Grid {
  pub fn new(height: usize, width: usize) -> Grid {
    let mut vertices = Vec::new();
    for col_index in 0..width {
      for row_index in 0..height {
        let cell = Cell::new(Grid::cell_id_from_location(GridCoords {
          col_index,
          row_index
        }));
        vertices.push(cell);
      }
    }

    Grid {
      height,
      width,
      graph: GraphBuilder::new().with_vertices(vertices).build()
    }
  }

  fn cell_id_from_location(pos: GridCoords) -> <Cell as HasID>::ID_TYPE {
    format!("{}_{}", pos.row_index, pos.col_index)
  }

  fn location_from_cell_id(cell_id: &String) -> GridCoords {
    let parts: Vec<&str> = cell_id.as_str().split("_").collect();

    let row_index = parts[0].parse::<usize>().unwrap();
    let col_index = parts[1].parse::<usize>().unwrap();

    GridCoords {
      row_index,
      col_index
    }
  }

  pub fn cell_at(&self, pos: GridCoords) -> Option<&Cell> {
    let key = Grid::cell_id_from_location(pos);
    self.graph.vertices().get(&key)
  }

  fn _each_row<F>(&self, mut f: F, reverse: bool)
    where F: (FnMut(&[(&Cell, GridCoords)], usize))
  {
    if self.height == 0 {
      return;
    }
    let height_max_index = self.height - 1;
    for idx in 0..self.height {
      let row_index;
      if reverse {
        row_index = height_max_index - idx;
      } else {
        row_index = idx;
      }
      let row_cells: Vec<(&Cell, GridCoords)> = (0..self.width).map(|col_index| {
        let loc = GridCoords {
          col_index,
          row_index
        };
        let cell = self.cell_at(loc).unwrap();
        (cell, loc)
      }).collect();
      f(row_cells.as_slice(), idx);
    }
  }

  pub fn each_row<F>(&self, f: F)
    where F: (FnMut(&[(&Cell, GridCoords)], usize))
  {
    self._each_row(f, false);
  }

  pub fn each_row_reverse<F>(&self, f: F)
    where F: (FnMut(&[(&Cell, GridCoords)], usize))
  {
    self._each_row(f, true);
  }

  pub fn each_cell<F>(&self, mut f: F)
    where F: (FnMut((&Cell, GridCoords)))
  {
    for row_index in 0..self.height {
      for col_index in 0..self.width {
        let loc = GridCoords {
          col_index,
          row_index
        };
        let cell = self.cell_at(loc).unwrap();
        f((cell, loc));
      }
    }
  }

  pub fn rand_cell(&self) -> &Cell {
     let row_index = rand::thread_rng().gen_range(0, self.width);
     let col_index = rand::thread_rng().gen_range(0, self.height);
     // we can unwrap here since the calculation is
     // bound to the size of the grid
     self.cell_at(GridCoords {
        row_index,
        col_index
     }).unwrap()
  }

  pub fn neighbors(&self, pos: GridCoords) -> Neighbors {
    let mut neighbors = Neighbors::new();
    let neighbor_positions = vec![
      ("north", self.north(pos)),
      ("east", self.east(pos)),
      ("south", self.south(pos)),
      ("west", self.west(pos))
    ];
    for neighbor_position in neighbor_positions {
      let (dir, neighbor_position) = neighbor_position;
      if let GridPosition::InBounds(loc) = neighbor_position {
        if let Some(cell) = self.cell_at(loc) {
          neighbors.set(dir, Neighbor::NeighborCell((loc, cell)));
        }
      }
    }
    neighbors
  }

  pub fn links(&self, pos: GridCoords) -> &[Edge<Cell, EdgeData>] {
    let id = Grid::cell_id_from_location(pos);
    self.graph.get_adjacent(&id)
  }

  pub fn link(&mut self, source: GridCoords, destination: GridCoords) {
    let s = Grid::cell_id_from_location(source);
    let d = Grid::cell_id_from_location(destination);
    self.graph.add_edge(&s, &d);
  }

  pub fn link_bidi(&mut self, source: GridCoords, destination: GridCoords) {
    let s = Grid::cell_id_from_location(source);
    let d = Grid::cell_id_from_location(destination);
    self.graph.add_edge(&s, &d);
    self.graph.add_edge(&d, &s);
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn north(&self, pos: GridCoords) -> GridPosition {
    if pos.row_index == usize::max_value() || (pos.row_index + 1) == self.height {
      return GridPosition::OutOfBounds
    }
    GridPosition::InBounds(GridCoords {
      row_index: pos.row_index + 1,
      ..pos
    })
  }

  pub fn east(&self, pos: GridCoords) -> GridPosition {
    if pos.col_index == usize::max_value() || (pos.col_index + 1) == self.width {
      return GridPosition::OutOfBounds
    }
    GridPosition::InBounds(GridCoords {
      col_index: pos.col_index + 1,
      ..pos
    })
  }

  pub fn south(&self, pos: GridCoords) -> GridPosition {
    if pos.row_index == usize::min_value() {
      return GridPosition::OutOfBounds
    }
    GridPosition::InBounds(GridCoords {
      row_index: pos.row_index - 1,
      ..pos
    })
  }

  pub fn west(&self, pos: GridCoords) -> GridPosition {
    if pos.col_index == usize::min_value() {
      return GridPosition::OutOfBounds
    }
    GridPosition::InBounds(GridCoords {
      col_index: pos.col_index - 1,
      ..pos
    })
  }

  fn draw_line(mut img: image::RgbImage, color: image::Rgb<u8>, (x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> image::RgbImage {
    let p1 = bresenham::Point::new(x1, y1);
    let p2 = bresenham::Point::new(x2, y2);
    let line = bresenham::calculate_line(p1, p2);
    for point in line {
      img.put_pixel(point.x, point.y, color);
    };
    img
  }

  pub fn to_img(&self, path: &str, cell_size: u32) {
    let padding_px = 5;
    let padding_total = padding_px * 2;

    let grid_width_u32;
    if let Ok(width_u32) = u32::try_from(self.width) {
      grid_width_u32 = width_u32
    } else {
      panic!("Grid width is too large to convert into an image (u32 max)")
    }

    let grid_height_u32;
    if let Ok(height_u32) = u32::try_from(self.width) {
      grid_height_u32 = height_u32
    } else {
      panic!("Grid height is too large to convert into an image (u32 max)")
    }

    let grid_width = (grid_width_u32 * cell_size) + padding_total;
    let grid_height = (grid_height_u32 * cell_size) + padding_total;

    let white = image::Rgb { data: [255, 255, 255] };
    let black = image::Rgb { data: [0, 0, 0] };

    let mut img: image::RgbImage = image::ImageBuffer::from_pixel(grid_width, grid_height, white);
    for row_index in 0..self.height {
      for col_index in 0..self.width {
        let coords = GridCoords {
          col_index,
          row_index
        };
        let neighbors = self.neighbors(coords.to_owned());
        let links = self.links(coords.to_owned());

        let origin_x = (u32::try_from(coords.col_index).unwrap() * cell_size) + padding_px;
        let origin_y = (u32::try_from(coords.row_index).unwrap() * cell_size) + padding_px;
        let left_wall_y = origin_y + cell_size;
        let bot_wall_x = origin_x + cell_size;

        match neighbors.west {
          Neighbor::GridBoundary => {
            img = Grid::draw_line(img, black, (origin_x, origin_y), (origin_x, left_wall_y));
          },
          Neighbor::NeighborCell((neighbor_coords, _)) => {
            if !links.iter().any(|link| Grid::location_from_cell_id(&link.to) == neighbor_coords) {
              img = Grid::draw_line(img, black, (origin_x, origin_y), (origin_x, left_wall_y));
            }
          }
        }

        match neighbors.south {
          Neighbor::GridBoundary => {
            img = Grid::draw_line(img, black, (origin_x, origin_y), (bot_wall_x, origin_y));
          },
          Neighbor::NeighborCell((neighbor_coords, _)) => {
            if !links.iter().any(|link| Grid::location_from_cell_id(&link.to) == neighbor_coords) {
              img = Grid::draw_line(img, black, (origin_x, origin_y), (bot_wall_x, origin_y));
            }
          }
        }
      }
    }

    let top_y = padding_px + (grid_height_u32 * cell_size);
    let top_x = padding_px;
    let top_x2 = padding_px + (grid_width_u32 * cell_size);
    img = Grid::draw_line(img, black, (top_x, top_y), (top_x2, top_y));

    let right_y = padding_px;
    let right_x = padding_px + (grid_width_u32 * cell_size);
    let right_y2 = padding_px + (grid_height_u32 * cell_size);
    img = Grid::draw_line(img, black, (right_x, right_y), (right_x, right_y2));

    img = image::imageops::flip_vertical(&img);

    img.save(path).unwrap();
  }
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let top_corner = String::from("+");
    let top_border = String::from("---+").repeat(self.width);
    let mut lines = vec![format!("{}{}", top_corner, top_border)];

    self.each_row_reverse(|row, _| {
      let mut top = String::new();
      let mut bottom = String::new();

      for (_, coords) in row {
        let neighbors = self.neighbors(coords.to_owned());
        let links = self.links(coords.to_owned());

        match neighbors.west {
          Neighbor::GridBoundary => {
            top.push_str("|");
          },
          Neighbor::NeighborCell((neighbor_coords, _)) => {
            if links.iter().any(|link| Grid::location_from_cell_id(&link.to) == neighbor_coords) {
              top.push_str(" ");
            } else {
              top.push_str("|");
            }
          }
        }
        bottom.push_str("+");

        top.push_str("   ");
        match neighbors.south {
          Neighbor::GridBoundary => {
            bottom.push_str("---");
          },
          Neighbor::NeighborCell((neighbor_coords, _)) => {
            if links.iter().any(|link| Grid::location_from_cell_id(&link.to) == neighbor_coords) {
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
    });

    for line in lines {
      if let Result::Err(e) = writeln!(f, "{}", line) {
        panic!(e);
      }
    }

    Ok(())
  }
}
