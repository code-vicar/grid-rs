use crate::grid::Grid;
use crate::grid::cell::GridCell;
use rand::Rng;
use super::CoinFlip;

pub fn apply_to<C: GridCell + std::fmt::Debug>(mut grid: Grid<C>) -> Grid<C> {
  for id in grid.cell_ids() {
    let cell = grid.cell_at(&id).unwrap();
    let north = grid.north_id(cell);
    let east = grid.east_id(cell);
    match rand::thread_rng().gen::<CoinFlip>() {
      CoinFlip::Heads => {
        // try north first
        if let Some(north_id) = north {
          grid.link_bidi(&id, &north_id);
        } else if let Some(east_id) = east {
          grid.link_bidi(&id, &east_id);
        }
      }
      CoinFlip::Tails => {
        // try east first
        if let Some(east_id) = east {
          grid.link_bidi(&id, &east_id);
        } else if let Some(north_id) = north {
          grid.link_bidi(&id, &north_id);
        }
      }
    }
  }
  grid
}
