pub mod cell;

use std::collections::HashMap;
use std::fmt;
use std::convert::TryFrom;
use gust::{Graph, Edge};
use line_rs::*;
use rand::Rng;
use cell::{HasID, GridCoords, GridCell};

#[derive(Debug)]
pub struct Neighbors<'a, C: GridCell + std::fmt::Debug> {
  pub north: Option<&'a C>,
  pub east: Option<&'a C>,
  pub south: Option<&'a C>,
  pub west: Option<&'a C>
}

#[derive(Debug)]
pub struct Grid<C: GridCell + std::fmt::Debug> {
  height: usize,
  width: usize,
  cells: HashMap<<C as HasID>::ID_TYPE, C>,
  graph: Graph<C>,
}

impl<C: GridCell + std::fmt::Debug> Grid<C> {
  pub fn new(height: usize, width: usize) -> Grid<C> {
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
        let cell = C::new_from_id(C::coords_as_id(coords));
        grid.cells.insert(cell.get_id().clone(), cell);
      }
    }
    grid
  }

  pub fn cells(&self) -> &HashMap<<C as HasID>::ID_TYPE, C> {
    &self.cells
  }

  pub fn cell_ids(&self) -> Vec<<C as HasID>::ID_TYPE> {
    self.cells.iter().map(|(cell_id, _)| cell_id.clone()).collect()
  }

  pub fn cell_at(&self, cell_id: &<C as HasID>::ID_TYPE) -> Option<&C> {
    self.cells.get(cell_id)
  }

  fn _each_row(&self, reverse: bool) -> Vec<Vec<<C as HasID>::ID_TYPE>> {
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
      let mut ids = Vec::new();
      for col_index in 0..self.width {
        ids.push(C::coords_as_id(GridCoords {
          col_index,
          row_index
        }));
      }
      rows.push(ids);
    }
    rows
  }

  pub fn rows(&self) -> Vec<Vec<<C as HasID>::ID_TYPE>> {
    self._each_row(false)
  }

  pub fn rows_reverse(&self) -> Vec<Vec<<C as HasID>::ID_TYPE>> {
    self._each_row(true)
  }

  pub fn rand_cell(&self) -> &C {
     let row_index = rand::thread_rng().gen_range(0, self.width);
     let col_index = rand::thread_rng().gen_range(0, self.height);
     // we can unwrap here since the calculation is
     // bound to the size of the grid
     self.cell_at(&C::coords_as_id(GridCoords {
        row_index,
        col_index
     })).unwrap()
  }

  pub fn neighbors(&self, cell: &C) -> Neighbors<C> {
    Neighbors {
      north: self.north(cell),
      east: self.east(cell),
      south: self.south(cell),
      west: self.west(cell),
    }
  }

  pub fn links(&self, cell: &C) -> Vec<&Edge<C>> {
    self.graph.get_adjacent(cell.get_id())
  }

  pub fn link(&mut self, source: &C::ID_TYPE, destination: &C::ID_TYPE) {
    self.graph.add_edge(source.clone(), destination.clone());
  }

  pub fn link_bidi(&mut self, source: &C::ID_TYPE, destination: &C::ID_TYPE) {
    self.graph.add_edge_bidi(source.clone(), destination.clone());
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn north(&self, cell: &C) -> Option<&C> {
    let north_coords = cell.north_coords();
    match north_coords {
      Some(coords) => {
        if coords.row_index >= self.height {
          return None
        }
        self.cell_at(&C::coords_as_id(coords))
      }
      None => None
    }
  }

  pub fn north_id(&self, cell: &C) -> Option<C::ID_TYPE> {
    match self.north(cell) {
      Some(cell) => {
        Some(cell.get_id().clone())
      }
      None => None
    }
  }

  pub fn east(&self, cell: &C) -> Option<&C> {
    let east_coords = cell.east_coords();
    match east_coords {
      Some(coords) => {
        if coords.col_index >= self.width {
          return None
        }
        self.cell_at(&C::coords_as_id(coords))
      }
      None => None
    }
  }

  pub fn east_id(&self, cell: &C) -> Option<C::ID_TYPE> {
    match self.east(cell) {
      Some(cell) => {
        Some(cell.get_id().clone())
      }
      None => None
    }
  }

  pub fn south(&self, cell: &C) -> Option<&C> {
    let south_coords = cell.south_coords();
    match south_coords {
      Some(coords) => {
        self.cell_at(&C::coords_as_id(coords))
      }
      None => None
    }
  }

  pub fn south_id(&self, cell: &C) -> Option<C::ID_TYPE> {
    match self.south(cell) {
      Some(cell) => {
        Some(cell.get_id().clone())
      }
      None => None
    }
  }

  pub fn west(&self, cell: &C) -> Option<&C> {
    let west_coords = cell.west_coords();
    match west_coords {
      Some(coords) => {
        self.cell_at(&C::coords_as_id(coords))
      }
      None => None
    }
  }

  pub fn west_id(&self, cell: &C) -> Option<C::ID_TYPE> {
    match self.west(cell) {
      Some(cell) => {
        Some(cell.get_id().clone())
      }
      None => None
    }
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
    for (cell_id, cell) in self.cells.iter() {
      let neighbors = self.neighbors(cell);
      let links = self.links(cell);

      let coords = cell.coords();
      let origin_x = (u32::try_from(coords.col_index).unwrap() * cell_size) + padding_px;
      let origin_y = (u32::try_from(coords.row_index).unwrap() * cell_size) + padding_px;
      let left_wall_y = origin_y + cell_size;
      let bot_wall_x = origin_x + cell_size;

      match neighbors.west {
        Some(west) => {
          if !links.iter().any(|link| link.leads_from_to(cell_id, west.get_id())) {
            img = Self::draw_line(img, black, (origin_x, origin_y), (origin_x, left_wall_y));
          }
        },
        None => {
          img = Self::draw_line(img, black, (origin_x, origin_y), (origin_x, left_wall_y));
        }
      }

      match neighbors.south {
        Some(south) => {
          if !links.iter().any(|link| link.leads_from_to(cell_id, south.get_id())) {
            img = Self::draw_line(img, black, (origin_x, origin_y), (bot_wall_x, origin_y));
          }
        },
        None => {
          img = Self::draw_line(img, black, (origin_x, origin_y), (bot_wall_x, origin_y));
        }
      }
    }

    let top_y = padding_px + (grid_height_u32 * cell_size);
    let top_x = padding_px;
    let top_x2 = padding_px + (grid_width_u32 * cell_size);
    img = Self::draw_line(img, black, (top_x, top_y), (top_x2, top_y));

    let right_y = padding_px;
    let right_x = padding_px + (grid_width_u32 * cell_size);
    let right_y2 = padding_px + (grid_height_u32 * cell_size);
    img = Self::draw_line(img, black, (right_x, right_y), (right_x, right_y2));

    img = image::imageops::flip_vertical(&img);

    img.save(path).unwrap();
  }
}

impl<C: GridCell + std::fmt::Debug> fmt::Display for Grid<C> {
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
            if links.iter().any(|link| link.leads_to(cell.get_id())) {
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
            if links.iter().any(|link| link.leads_to(cell.get_id())) {
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
