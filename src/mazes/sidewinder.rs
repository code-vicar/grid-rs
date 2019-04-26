use crate::grid::{Grid, GridCoords, GridPosition};
use super::CoinFlip;
use rand::prelude::*;

fn link_run(to_link: &mut Vec<(GridCoords, GridCoords)>, run: &Vec<GridCoords>) {
  for i in 1..run.len() {
    let from = run.get(i - 1).unwrap().to_owned();
    let to = run.get(i).unwrap().to_owned();
    to_link.push((from, to));
  }
}

fn close_the_run(grid: &Grid, to_link: &mut Vec<(GridCoords, GridCoords)>, run: &mut Vec<GridCoords>) {
  link_run(to_link, &run);
  let mut rng = thread_rng();
  let run_idx = rng.gen_range(0, run.len());
  let run_coords = run.get(run_idx).unwrap().to_owned();
  // link a random cell in the run to the north
  to_link.push((run_coords, grid.north(run_coords).unwrap()));
  // clear this run
  run.clear();
}

pub fn apply_to(grid: Grid) -> Grid {
  let mut to_link: Vec<(GridCoords, GridCoords)> = Vec::new();
  let top_row_idx;

  if grid.height() > 0 {
    top_row_idx = grid.height() - 1;
  } else {
    top_row_idx = grid.height();
  }

  grid.each_row(|row, row_idx| {
    let mut run: Vec<GridCoords> = Vec::new();
    // iterate over cells in the row
    for (_, row_coords) in row {
      let current_coords = row_coords.to_owned();
      run.push(current_coords); // add current cell to the run
      if row_idx == top_row_idx {
        continue; // top row can't close runs so just skip that part
      }
      let east_pos = grid.east(current_coords); // lookup the cell to the east
      match east_pos {
        GridPosition::OutOfBounds => {
          close_the_run(&grid, &mut to_link, &mut run); // if it's out of bounds then end the run
        }
        GridPosition::InBounds(_) => {
          if let CoinFlip::Heads = rand::thread_rng().gen::<CoinFlip>() {
            close_the_run(&grid, &mut to_link, &mut run); // if we flip heads then end the run
          }
        }
      }
    }
    // link any remaining run from the last processed row
    link_run(&mut to_link, &run);
  });

  let mut grid = grid;
  for (source, dest) in to_link {
    grid.link_bidi(source, dest);
  }
  grid
}
