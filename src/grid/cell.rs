pub use gust::traits::HasID;
// use std::fmt;

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

// impl fmt::Display for Cell {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{}", self.get_id())
//   }
// }
