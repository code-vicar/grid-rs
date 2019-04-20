use crate::grid::{Grid, GridCoords, GridPosition};
use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
enum CoinFlip {
  Heads,
  Tails
}

impl Distribution<CoinFlip> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CoinFlip {
        if rng.gen::<bool>() {
          CoinFlip::Heads
        } else {
          CoinFlip::Tails
        }
    }
}

pub fn apply_to(grid: Grid) -> Grid {
  let mut to_link: Vec<(GridCoords, GridCoords)> = Vec::new();
  grid.each_cell(|(_, current_coords)| {
    let north_pos = grid.north(current_coords);
    let east_pos = grid.east(current_coords);
    match rand::thread_rng().gen::<CoinFlip>() {
      CoinFlip::Heads => {
        // try north first
        if let GridPosition::InBounds(north_coords) = north_pos {
          to_link.push((current_coords, north_coords));
        } else if let GridPosition::InBounds(east_coords) = east_pos {
          to_link.push((current_coords, east_coords));
        }
      }
      CoinFlip::Tails => {
        // try east first
        if let GridPosition::InBounds(east_coords) = east_pos {
          to_link.push((current_coords, east_coords));
        } else if let GridPosition::InBounds(north_coords) = north_pos {
          to_link.push((current_coords, north_coords));
        }
      }
    }
  });
  let mut grid = grid;
  for (source, dest) in to_link {
    grid.link_bidi(source, dest);
  }
  grid
}
