use std::collections::*;
use super::{Grid, GridCoords};
use std::convert::{TryInto, TryFrom};
use line_rs::*;

pub struct GridImage {
  pub canvas: image::RgbImage,
  pub cell_size: u32,
  pub padding: u32,
}

enum CellPoint {
  TopLeft,
  // TopRight,
  BottomRight,
  Center
}

fn get_origin(padding: u32, cell_size: u32, cell: &GridCoords) -> (u32, u32) {
  let origin_x = padding + (u32::try_from(cell.col_index).unwrap() * cell_size);
  let origin_y = padding + (u32::try_from(cell.row_index).unwrap() * cell_size);
  (origin_x, origin_y)
}

// get a point relative to origin
fn get_point(origin: (u32, u32), cell_size: u32, point: CellPoint) -> (u32, u32) {
  match point {
    CellPoint::TopLeft => {
      (origin.0, origin.1 + cell_size)
    }
    CellPoint::BottomRight => {
      (origin.0 + cell_size, origin.1)
    }
    // CellPoint::TopRight => {
    //   (origin.0 + cell_size, origin.1 + cell_size)
    // }
    CellPoint::Center => {
      (origin.0 + (cell_size / 2), origin.1 + (cell_size / 2))
    }
  }
}

fn draw_line(mut canvas: image::RgbImage, color: image::Rgb<u8>, (x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> image::RgbImage {
  let p1 = bresenham::Point::new(x1, y1);
  let p2 = bresenham::Point::new(x2, y2);
  let line = bresenham::calculate_line(p1, p2);
  for point in line {
    canvas.put_pixel(point.x, point.y, color);
  };
  canvas
}

fn fill_square(mut canvas: image::RgbImage, color: image::Rgb<u8>, origin: (u32, u32), cell_size: u32) -> image::RgbImage {
  for px_x in 0..cell_size {
    for px_y in 0..cell_size {
      canvas.put_pixel(origin.0 + px_x, origin.1 + px_y, color);
    }
  }
  canvas
}

fn modify_color_by_distance(max_distance: u32, distance: u32, color: image::Rgb<u8>) -> image::Rgb<u8> {
  let fraction = (f64::try_from(distance).unwrap() / f64::try_from(max_distance).unwrap()) as f32;
  let fraction = 1.0 - fraction;
  let [r, g, b] = color.data;
  let r_float = f32::try_from(r).unwrap();
  let g_float = f32::try_from(g).unwrap();
  let b_float = f32::try_from(b).unwrap();
  let r_graded = ((r_float * fraction).trunc() as i32).try_into().unwrap();
  let g_graded = ((g_float * fraction).trunc() as i32).try_into().unwrap();
  let b_graded = ((b_float * fraction).trunc() as i32).try_into().unwrap();
  image::Rgb([r_graded, g_graded, b_graded])
}

pub fn to_img(grid: &Grid, cell_size: u32) -> GridImage {
  let padding_px = 5;
  let padding_total = padding_px * 2;

  let grid_width_u32;
  if let Ok(width_u32) = u32::try_from(grid.width) {
    grid_width_u32 = width_u32
  } else {
    panic!("Grid width is too large to convert into an image (u32 max)")
  }

  let grid_height_u32;
  if let Ok(height_u32) = u32::try_from(grid.width) {
    grid_height_u32 = height_u32
  } else {
    panic!("Grid height is too large to convert into an image (u32 max)")
  }

  let grid_width = (grid_width_u32 * cell_size) + padding_total;
  let grid_height = (grid_height_u32 * cell_size) + padding_total;

  let white = image::Rgb { data: [255, 255, 255] };
  let black = image::Rgb { data: [0, 0, 0] };

  let mut canvas: image::RgbImage = image::ImageBuffer::from_pixel(grid_width, grid_height, white);
  for (_, cell) in grid.cells.iter() {
    let neighbors = grid.neighbors(cell);
    let links = grid.links(cell);

    let coords = cell.coords();
    let origin = get_origin(padding_px, cell_size, coords);
    let top_left = get_point(origin, cell_size, CellPoint::TopLeft);
    let bottom_right = get_point(origin, cell_size, CellPoint::BottomRight);

    match neighbors.west {
      Some(west) => {
        if !links.iter().any(|linked_cell| *linked_cell == west) {
          canvas = draw_line(canvas, black, origin, top_left);
        }
      },
      None => {
        canvas = draw_line(canvas, black, origin, top_left);
      }
    }

    match neighbors.south {
      Some(south) => {
        if !links.iter().any(|linked_cell| *linked_cell == south) {
          canvas = draw_line(canvas, black, origin, bottom_right);
        }
      },
      None => {
        canvas = draw_line(canvas, black, origin, bottom_right);
      }
    }
  }

  let top_y = padding_px + (grid_height_u32 * cell_size);
  let top_x = padding_px;
  let top_x2 = padding_px + (grid_width_u32 * cell_size);
  canvas = draw_line(canvas, black, (top_x, top_y), (top_x2, top_y));

  let right_y = padding_px;
  let right_x = padding_px + (grid_width_u32 * cell_size);
  let right_y2 = padding_px + (grid_height_u32 * cell_size);
  canvas = draw_line(canvas, black, (right_x, right_y), (right_x, right_y2));

  canvas = image::imageops::flip_vertical(&canvas);

  GridImage {
    canvas,
    cell_size,
    padding: padding_px,
  }
}

pub fn draw_solution(mut grid_image: GridImage, solution: &Vec<GridCoords>) -> GridImage {
  let padding_px = grid_image.padding;
  let cell_size = grid_image.cell_size;
  let green = image::Rgb { data: [120, 255, 120] };

  let mut canvas = image::imageops::flip_vertical(&grid_image.canvas);

  let mut trailing_point = None;
  for coords in solution {
    let origin = get_origin(padding_px, cell_size, coords);
    let center_point = get_point(origin, cell_size, CellPoint::Center);
    canvas.put_pixel(center_point.0, center_point.1, green);
    if let Some(trailing_center_point) = trailing_point {
      canvas = draw_line(canvas, green, trailing_center_point, center_point);
    }
    trailing_point = Some(center_point);
  }
  grid_image.canvas = image::imageops::flip_vertical(&canvas);
  grid_image
}

pub fn draw_distance_gradation(mut grid_image: GridImage, max_distance: u32, distances: &HashMap<GridCoords, u32>, color: image::Rgb<u8>) -> GridImage {
  let padding_px = grid_image.padding;
  let cell_size = grid_image.cell_size;

  let mut canvas = image::imageops::flip_vertical(&grid_image.canvas);
  for (coords, distance) in distances {
    let graded_color = modify_color_by_distance(max_distance, distance.to_owned(), color);
    let origin = get_origin(padding_px, cell_size, coords);
    canvas = fill_square(canvas, graded_color, origin, cell_size);
  }
  grid_image.canvas = image::imageops::flip_vertical(&canvas);
  grid_image
}
