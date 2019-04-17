use gust::{Graph, GraphBuilder};

use super::cell::*;

#[derive(Debug)]
pub struct Edge {
}

#[derive(Debug)]
pub struct Grid {
  height: usize,
  width: usize,
  graph: Graph<Cell, Edge>,
}

impl Grid {
  pub fn new(height: usize, width: usize) -> Grid {
    let mut vertices = Vec::new();
    for col_index in 0..height {
      for row_index in 0..width {
        let key = format!("{}_{}", col_index, row_index);
        let cell = Cell::new(key.clone());
        // area_map.insert(key, &cell);
        vertices.push(cell);
      }
    }

    Grid {
      height,
      width,
      graph: GraphBuilder::new().with_vertices(vertices).build()
    }
  }

  pub fn cell_at(&self, col_index: usize, row_index: usize) -> Option<&Cell> {
    let key = format!("{}_{}", col_index, row_index);
    self.graph.vertices().get(&key)
  }

  pub fn each_cell<F>(&self, f: F)
    where F: (FnMut(&Cell))
  {
    let g = self.graph.vertices();
    g.values().for_each(f)
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn width(&self) -> usize {
    self.width
  }
}
