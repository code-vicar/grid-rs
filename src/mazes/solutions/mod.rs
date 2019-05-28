pub mod dijkstra {
  use std::collections::*;

  use crate::grid::cell::*;
  use crate::grid::Grid;

  #[derive(Debug)]
  pub struct Dijkstra<C: GridCell> {
    distances: HashMap<<C as HasID>::ID_TYPE, usize>,
    origin: <C as HasID>::ID_TYPE,
  }

  impl<C: GridCell> Dijkstra<C> {
    pub fn new(grid: &Grid<C>, origin: &<C as HasID>::ID_TYPE) -> Dijkstra<C> {
      let mut distances = HashMap::new();
      let mut frontier: VecDeque<&<C as HasID>::ID_TYPE> = VecDeque::new();
      let mut visited = HashSet::new();
      frontier.push_back(origin);
      distances.insert(origin.clone(), 0);
      while let Some(cell_id) = frontier.pop_front() {
        visited.insert(cell_id);
        let current_distance = distances.get(cell_id).unwrap();
        let next_distance = current_distance + 1;
        let cell = grid.cell_at(cell_id).expect(format!("No cell found with id, {:?}", cell_id).as_ref());
        for to in grid.links(cell) {
          if !visited.contains(to) {
            distances.insert(to.clone(), next_distance);
            frontier.push_back(to);
          }
        }
      }
      Dijkstra {
        distances,
        origin: origin.clone(),
      }
    }

    pub fn shortest_path_to<'a>(&self, grid: &Grid<C>, dest: &<C as HasID>::ID_TYPE) -> Vec<<C as HasID>::ID_TYPE> {
      let mut path = Vec::new();
      let mut next = Some(dest);
      let mut visited = HashSet::new();
      if &self.origin == dest {
        path.push(dest.clone());
        return path;
      }
      while let Some(id) = next {
        path.push(id.clone());
        visited.insert(id);
        next = None;
        let cell = grid.cell_at(&id).unwrap();
        let mut min_dist = &usize::max_value();
        for linked_cell_id in grid.links(cell) {
          if !visited.contains(&linked_cell_id) {
            let distance = self.distances.get(&linked_cell_id).unwrap();
            if distance <= min_dist {
              min_dist = distance;
              next = Some(linked_cell_id);
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
