pub use gust::traits::HasID;

#[derive(Debug, HasID)]
pub struct Cell {
  #[gust(id)]
  id: String
}

impl Cell {
  pub fn new(id: String) -> Cell {
    Cell {
      id
    }
  }
}
