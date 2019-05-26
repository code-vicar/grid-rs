pub use gust::traits::HasID;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct GridCoords {
  pub col_index: usize,
  pub row_index: usize
}

pub trait GridCell: HasID {
  fn new_from_id(id: Self::ID_TYPE) -> Self;
  fn coords_as_id(coords: GridCoords) -> Self::ID_TYPE;
  fn coords(&self) -> &GridCoords;
  fn north_coords(&self) -> Option<GridCoords>;
  fn east_coords(&self) -> Option<GridCoords>;
  fn south_coords(&self) -> Option<GridCoords>;
  fn west_coords(&self) -> Option<GridCoords>;
}

#[derive(Debug, HasID)]
pub struct Cell {
  #[gust(id)]
  id: GridCoords
}

impl GridCell for Cell {
  fn new_from_id(id: Self::ID_TYPE) -> Cell {
    Cell {
      id,
    }
  }

  fn coords_as_id(coords: GridCoords) -> Self::ID_TYPE {
    coords
  }

  fn coords(&self) -> &GridCoords {
    &self.id
  }

  fn north_coords(&self) -> Option<GridCoords> {
    let coords = self.get_id();
    if coords.row_index == usize::max_value() {
      return None;
    }
    Some(GridCoords {
      row_index: coords.row_index + 1,
      col_index: coords.col_index,
    })
  }

  fn east_coords(&self) -> Option<GridCoords> {
    let coords = self.get_id();
    if coords.col_index == usize::max_value() {
      return None;
    }
    Some(GridCoords {
      row_index: coords.row_index,
      col_index: coords.col_index + 1,
    })
  }

  fn south_coords(&self) -> Option<GridCoords> {
    let coords = self.get_id();
    if coords.row_index == usize::min_value() {
      return None;
    }
    Some(GridCoords {
      row_index: coords.row_index - 1,
      col_index: coords.col_index,
    })
  }

  fn west_coords(&self) -> Option<GridCoords> {
    let coords = self.get_id();
    if coords.col_index == usize::min_value() {
      return None;
    }
    Some(GridCoords {
      row_index: coords.row_index,
      col_index: coords.col_index - 1,
    })
  }
}
