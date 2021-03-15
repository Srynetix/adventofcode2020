//! Conway 3D

use std::collections::HashSet;

use super::common::Cell;
use crate::common::Vec3;

/// Game of Life in an 'infinite' 3D grid
#[derive(Debug, Default)]
pub struct Conway3D {
    map: HashSet<Vec3>,
    buffer: HashSet<Vec3>,
}

impl Conway3D {
    /// Creates new game.
    pub fn new() -> Self {
        Self {
            map: HashSet::new(),
            buffer: HashSet::new(),
        }
    }

    /// Set cell state at position.
    ///
    /// # Arguments
    ///
    /// * `position` - Grid position (x, y, z)
    /// * `state` - Cell state
    pub fn set_cell_state(&mut self, position: Vec3, state: Cell) {
        match state {
            Cell::Active => {
                self.map.insert(position);
                self.buffer.insert(position);
            }
            Cell::Inactive => {
                self.map.remove(&position);
                self.buffer.remove(&position);
            }
        }
    }

    /// Set multiple cell states.
    ///
    /// # Arguments
    ///
    /// * `stream` - `(Vec3, Cell)` tuple vec
    pub fn set_cell_states(&mut self, stream: Vec<(Vec3, Cell)>) {
        for (position, state) in stream {
            self.set_cell_state(position, state);
        }
    }

    /// Get cell at position.
    ///
    /// # Arguments
    ///
    /// * `position` - Position
    pub fn get_cell_at_position(&self, position: Vec3) -> Cell {
        self.map
            .get(&position)
            .map_or(Cell::Inactive, |_| Cell::Active)
    }

    /// Get active neighbors count from position.
    ///
    /// # Arguments
    ///
    /// * `position` - Position
    pub fn get_active_neighbors_count(&self, position: Vec3) -> usize {
        let mut count = 0;
        let (x, y, z) = position.into();

        for &i in &[-1, 0, 1] {
            for &j in &[-1, 0, 1] {
                for &k in &[-1, 0, 1] {
                    // Ignore current position
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }

                    if let Cell::Active = self.get_cell_at_position((x + i, y + j, z + k).into()) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Returns minimum and maximum bounds.
    pub fn get_bounds(&self) -> (Vec3, Vec3) {
        let (mut min_x, mut min_y, mut min_z) = (isize::MAX, isize::MAX, isize::MAX);
        let (mut max_x, mut max_y, mut max_z) = (isize::MIN, isize::MIN, isize::MIN);

        for p in &self.map {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            min_z = min_z.min(p.z);

            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
            max_z = max_z.max(p.z);
        }

        ((min_x, min_y, min_z).into(), (max_x, max_y, max_z).into())
    }

    /// Count active cells.
    pub fn count_active_cells(&self) -> usize {
        self.map.len()
    }

    /// Execute a simulation step.
    pub fn step(&mut self) {
        let (min_bounds, max_bounds) = self.get_bounds();

        for z in min_bounds.z - 1..=max_bounds.z + 1 {
            for y in min_bounds.y - 1..=max_bounds.y + 1 {
                for x in min_bounds.x - 1..=max_bounds.x + 1 {
                    let position = (x, y, z).into();
                    let old_state = self.get_cell_at_position(position);
                    let neighbors_count = self.get_active_neighbors_count(position);

                    let new_state = match old_state {
                        Cell::Active if !(2..=3).contains(&neighbors_count) => Cell::Inactive,
                        Cell::Inactive if neighbors_count == 3 => Cell::Active,
                        _ => old_state,
                    };

                    match new_state {
                        Cell::Active => {
                            self.buffer.insert(position);
                        }
                        Cell::Inactive => {
                            self.buffer.remove(&position);
                        }
                    }
                }
            }
        }

        self.map.clear();
        for k in &self.buffer {
            self.map.insert(*k);
        }
    }

    /// Run multiple steps.
    ///
    /// # Arguments
    ///
    /// * `n` - Step count
    pub fn run_steps(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }
}

impl From<&str> for Conway3D {
    #[allow(clippy::cast_possible_wrap)]
    fn from(value: &str) -> Self {
        let mut game = Self::new();

        game.set_cell_states(
            value
                .trim()
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.trim()
                        .chars()
                        .enumerate()
                        .map(|(x, c)| ((x as isize, y as isize, 0).into(), c.into()))
                        .collect::<Vec<(Vec3, Cell)>>()
                })
                .collect(),
        );

        game
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        .#.
        ..#
        ###
    "};

    #[test]
    fn test_set_cell_states_from_str() {
        let game = Conway3D::from(SAMPLE);
        assert_eq!(game.get_cell_at_position((0, 0, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((1, 0, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((2, 0, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((0, 1, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((1, 1, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((2, 1, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((0, 2, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((1, 2, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((2, 2, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((3, 2, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((0, 0, 1).into()), Cell::Inactive);
    }

    #[test]
    fn test_get_active_neighbors_count() {
        let game = Conway3D::from(SAMPLE);
        assert_eq!(game.get_active_neighbors_count((0, 0, 0).into()), 1);
        assert_eq!(game.get_active_neighbors_count((1, 1, 0).into()), 5);
        assert_eq!(game.get_active_neighbors_count((1, 1, 1).into()), 5);
        assert_eq!(game.get_active_neighbors_count((2, 1, 1).into()), 4);
    }

    #[test]
    fn test_step() {
        let mut game = Conway3D::from(SAMPLE);
        assert_eq!(game.get_bounds(), ((0, 0, 0).into(), (2, 2, 0).into()));

        game.step();
        assert_eq!(game.get_bounds(), ((0, 1, -1).into(), (2, 3, 1).into()));
        assert_eq!(game.count_active_cells(), 11);

        game.step();
        assert_eq!(game.count_active_cells(), 21);
    }

    #[test]
    fn test_run_6_steps() {
        let mut game = Conway3D::from(SAMPLE);
        game.run_steps(6);
        assert_eq!(game.count_active_cells(), 112);
    }
}
