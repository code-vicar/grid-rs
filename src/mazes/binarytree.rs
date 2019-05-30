use crate::grid::Grid;
use rand::Rng;
use super::CoinFlip;

pub fn apply_to(mut grid: Grid) -> Grid {
  for coords in grid.coords() {
    let cell = grid.cell_at(&coords).unwrap();
    let north = grid.north(cell);
    let east = grid.east(cell);
    match rand::thread_rng().gen::<CoinFlip>() {
      CoinFlip::Heads => {
        // try north first
        if let Some(north) = north {
          grid.link_bidi(&coords, &north.coords().clone());
        } else if let Some(east) = east {
          grid.link_bidi(&coords, &east.coords().clone());
        }
      }
      CoinFlip::Tails => {
        // try east first
        if let Some(east) = east {
          grid.link_bidi(&coords, &east.coords().clone());
        } else if let Some(north) = north {
          grid.link_bidi(&coords, &north.coords().clone());
        }
      }
    }
  }
  grid
}
