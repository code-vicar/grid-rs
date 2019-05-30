pub mod dijkstra {
  use std::collections::*;

  use crate::grid::cell::*;
  use crate::grid::Grid;

  #[derive(Debug)]
  pub struct Dijkstra {
    distances: HashMap<GridCoords, usize>,
    origin: GridCoords,
  }

  impl Dijkstra {
    pub fn new(grid: &Grid, origin: &GridCoords) -> Dijkstra {
      let mut distances = HashMap::new();
      let mut frontier: VecDeque<&GridCoords> = VecDeque::new();
      let mut visited = HashSet::new();
      frontier.push_back(origin);
      distances.insert(origin.clone(), 0);
      while let Some(coords) = frontier.pop_front() {
        visited.insert(coords);
        let current_distance = distances.get(coords).unwrap();
        let next_distance = current_distance + 1;
        let cell = grid.cell_at(coords).expect(format!("No cell found at coords, {:?}", coords).as_ref());
        for to in grid.links(cell) {
          if !visited.contains(to.coords()) {
            distances.insert(to.coords().clone(), next_distance);
            frontier.push_back(to.coords());
          }
        }
      }
      Dijkstra {
        distances,
        origin: origin.clone(),
      }
    }

    pub fn shortest_path_to<'a>(&self, grid: &Grid, dest: &GridCoords) -> Vec<GridCoords> {
      let mut path = Vec::new();
      let mut next = Some(dest);
      let mut visited = HashSet::new();
      while let Some(coords) = next {
        path.push(coords.clone());
        if &self.origin == coords {
          return path;
        }
        visited.insert(coords);
        next = None;
        let cell = grid.cell_at(coords).unwrap();
        let mut min_dist = &usize::max_value();
        for linked_cell in grid.links(cell) {
          if !visited.contains(linked_cell.coords()) {
            let distance = self.distances.get(linked_cell.coords()).unwrap();
            if distance <= min_dist {
              min_dist = distance;
              next = Some(linked_cell.coords());
            }
          }
        }
      }
      path
    }

    pub fn longest_path() {

    }
  }
}
