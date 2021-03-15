//! Conway 4D

use std::collections::HashSet;

use super::common::Cell;
use crate::common::Vec4;

/// Game of Life in an 'infinite' 4D grid
#[derive(Debug, Default)]
pub struct Conway4D {
    map: HashSet<Vec4>,
    buffer: HashSet<Vec4>,
}

impl Conway4D {
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
    /// * `position` - Grid position (x, y, z, t)
    /// * `state` - Cell state
    pub fn set_cell_state(&mut self, position: Vec4, state: Cell) {
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
    /// * `stream` - `(Vec4, Cell)` tuple vec
    pub fn set_cell_states(&mut self, stream: Vec<(Vec4, Cell)>) {
        for (position, state) in stream {
            self.set_cell_state(position, state);
        }
    }

    /// Get cell at position.
    ///
    /// # Arguments
    ///
    /// * `position` - Position
    pub fn get_cell_at_position(&self, position: Vec4) -> Cell {
        self.map
            .get(&position)
            .map_or(Cell::Inactive, |_| Cell::Active)
    }

    /// Get active neighbors count from position.
    ///
    /// # Arguments
    ///
    /// * `position` - Position
    pub fn get_active_neighbors_count(&self, position: Vec4) -> usize {
        let mut count = 0;
        let (x, y, z, t) = position.into();

        for &i in &[-1, 0, 1] {
            for &j in &[-1, 0, 1] {
                for &k in &[-1, 0, 1] {
                    for &m in &[-1, 0, 1] {
                        // Ignore current position
                        if i == 0 && j == 0 && k == 0 && m == 0 {
                            continue;
                        }

                        if let Cell::Active =
                            self.get_cell_at_position((x + i, y + j, z + k, t + m).into())
                        {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    /// Returns minimum and maximum bounds.
    pub fn get_bounds(&self) -> (Vec4, Vec4) {
        let (mut min_x, mut min_y, mut min_z, mut min_t) =
            (isize::MAX, isize::MAX, isize::MAX, isize::MAX);
        let (mut max_x, mut max_y, mut max_z, mut max_t) =
            (isize::MIN, isize::MIN, isize::MIN, isize::MIN);

        for p in &self.map {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            min_z = min_z.min(p.z);
            min_t = min_t.min(p.t);

            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
            max_z = max_z.max(p.z);
            max_t = max_t.max(p.t);
        }

        (
            (min_x, min_y, min_z, min_t).into(),
            (max_x, max_y, max_z, max_t).into(),
        )
    }

    /// Count active cells.
    pub fn count_active_cells(&self) -> usize {
        self.map.len()
    }

    /// Execute a simulation step.
    pub fn step(&mut self) {
        let (min_bounds, max_bounds) = self.get_bounds();

        for t in min_bounds.t - 1..=max_bounds.t + 1 {
            for z in min_bounds.z - 1..=max_bounds.z + 1 {
                for y in min_bounds.y - 1..=max_bounds.y + 1 {
                    for x in min_bounds.x - 1..=max_bounds.x + 1 {
                        let position = (x, y, z, t).into();
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
        }

        // Swap buffers
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

impl From<&str> for Conway4D {
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
                        .map(|(x, c)| ((x as isize, y as isize, 0, 0).into(), c.into()))
                        .collect::<Vec<(Vec4, Cell)>>()
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
        let game = Conway4D::from(SAMPLE);
        assert_eq!(
            game.get_cell_at_position((0, 0, 0, 0).into()),
            Cell::Inactive
        );
        assert_eq!(game.get_cell_at_position((1, 0, 0, 0).into()), Cell::Active);
        assert_eq!(
            game.get_cell_at_position((2, 0, 0, 0).into()),
            Cell::Inactive
        );
        assert_eq!(
            game.get_cell_at_position((0, 1, 0, 0).into()),
            Cell::Inactive
        );
        assert_eq!(
            game.get_cell_at_position((1, 1, 0, 0).into()),
            Cell::Inactive
        );
        assert_eq!(game.get_cell_at_position((2, 1, 0, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((0, 2, 0, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((1, 2, 0, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((2, 2, 0, 0).into()), Cell::Active);
        assert_eq!(
            game.get_cell_at_position((3, 2, 0, 0).into()),
            Cell::Inactive
        );
        assert_eq!(
            game.get_cell_at_position((0, 0, 1, 0).into()),
            Cell::Inactive
        );
    }

    #[test]
    fn test_run_6_steps() {
        let mut game = Conway4D::from(SAMPLE);
        game.run_steps(6);
        assert_eq!(game.count_active_cells(), 848);
    }
}
