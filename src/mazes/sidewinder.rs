use crate::grid::Grid;
use crate::grid::cell::GridCoords;
use super::CoinFlip;
use rand::prelude::*;

macro_rules! close_run {
  ($grid:ident, $run:ident) => {
    if $run.len() > 0 {
      for i in 1..$run.len() {
        let from = $run.get(i - 1).unwrap();
        let to = $run.get(i).unwrap();

        $grid.link_bidi(from, to);
      }
      let mut rng = thread_rng();
      let run_idx = rng.gen_range(0, $run.len());
      let run_cell_coords = $run.get(run_idx).unwrap();
      // link a random cell in the run to the north
      let north = $grid.north($grid.cell_at(run_cell_coords).unwrap());
      if let Some(north) = north {
        $grid.link_bidi(run_cell_coords, &north.coords().clone());
      }
      // clear this run
      $run.clear();
    }
  };
}

pub fn apply_to(mut grid: Grid) -> Grid {
  let top_row_idx;
  if grid.height() > 0 {
    top_row_idx = grid.height() - 1;
  } else {
    top_row_idx = grid.height();
  }
  let rows = grid.rows();
  let mut row_idx = 0;
  for ref row in rows {
    let mut run: Vec<&GridCoords> = Vec::new();
    // iterate over cells in the row
    for ref id in row {
      let cell = grid.cell_at(id).unwrap();
      run.push(id); // add current cell to the run
      if row_idx == top_row_idx {
        continue; // top row can't close runs so just skip that part
      }
      let east = grid.east(cell);
      match east {
        None => {
          // nowhere left to go but up
          close_run!(grid, run);
        }
        Some(_) => {
          // could continue east, flip a coin to see if we do
          if let CoinFlip::Heads = rand::thread_rng().gen::<CoinFlip>() {
            // nope, closing it out
            close_run!(grid, run);
          }
        }
      }
    }
    // close out any remaining run
    close_run!(grid, run);
    row_idx = row_idx + 1;
  }
  grid
}
