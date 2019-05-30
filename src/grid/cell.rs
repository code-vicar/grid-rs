pub use gust::traits::NodeID;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct GridCoords {
  pub col_index: usize,
  pub row_index: usize
}

#[derive(Debug, NodeID, PartialEq)]
#[gust(node_id=GridCoords)]
pub struct Cell {
  coords: GridCoords
}

// impl GridCell for Cell {
impl Cell {
  pub fn new(coords: &GridCoords) -> Cell {
    Cell {
      coords: coords.clone(),
    }
  }

  pub fn coords(&self) -> &GridCoords {
    &self.coords
  }

  pub fn north_coords(&self) -> Option<GridCoords> {
    if self.coords.row_index == usize::max_value() {
      return None;
    }
    Some(GridCoords {
      row_index: self.coords.row_index + 1,
      col_index: self.coords.col_index,
    })
  }

  pub fn east_coords(&self) -> Option<GridCoords> {
    if self.coords.col_index == usize::max_value() {
      return None;
    }
    Some(GridCoords {
      row_index: self.coords.row_index,
      col_index: self.coords.col_index + 1,
    })
  }

  pub fn south_coords(&self) -> Option<GridCoords> {
    if self.coords.row_index == usize::min_value() {
      return None;
    }
    Some(GridCoords {
      row_index: self.coords.row_index - 1,
      col_index: self.coords.col_index,
    })
  }

  pub fn west_coords(&self) -> Option<GridCoords> {
    if self.coords.col_index == usize::min_value() {
      return None;
    }
    Some(GridCoords {
      row_index: self.coords.row_index,
      col_index: self.coords.col_index - 1,
    })
  }
}
