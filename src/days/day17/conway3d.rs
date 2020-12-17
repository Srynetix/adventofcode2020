//! Conway 3D

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use super::common::{Cell, DayError, Vec3};

/// Game of Life in an 'infinite' 3D grid
#[derive(Debug, Default)]
pub struct Conway3D {
    map: HashMap<Vec3, Cell>,
    buffer: HashMap<Vec3, Cell>,
}

impl Conway3D {
    /// Creates new game.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            buffer: HashMap::new(),
        }
    }

    /// Set cell state at position.
    ///
    /// # Arguments
    ///
    /// * `position` - Grid position (x, y, z)
    /// * `state` - Cell state
    pub fn set_cell_state(&mut self, position: Vec3, state: Cell) {
        self.map.insert(position, state);
        self.buffer.insert(position, state);
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
        self.map.get(&position).cloned().unwrap_or(Cell::Inactive)
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
        (
            self.map.keys().min().copied().unwrap(),
            self.map.keys().max().copied().unwrap(),
        )
    }

    /// Count active cells.
    pub fn count_active_cells(&self) -> usize {
        self.map.values().filter(|&&x| x == Cell::Active).count()
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
                        Cell::Active if neighbors_count < 2 || neighbors_count > 3 => {
                            Cell::Inactive
                        }
                        Cell::Inactive if neighbors_count == 3 => Cell::Active,
                        _ => old_state,
                    };

                    self.buffer.insert(position, new_state);
                }
            }
        }

        // Swap buffers
        for (k, v) in &self.buffer {
            self.map.insert(*k, *v);
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

impl TryFrom<&str> for Conway3D {
    type Error = DayError;

    #[allow(clippy::cast_possible_wrap)]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut game = Self::new();

        game.set_cell_states(
            value
                .trim()
                .lines()
                .enumerate()
                .map(|(y, l)| {
                    Ok(l.trim()
                        .chars()
                        .enumerate()
                        .map(|(x, c)| Ok(((x as isize, y as isize, 0).into(), c.try_into()?)))
                        .collect::<Result<Vec<(Vec3, Cell)>, _>>()?)
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect(),
        );

        Ok(game)
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
        let game = Conway3D::try_from(SAMPLE).unwrap();
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
        let game = Conway3D::try_from(SAMPLE).unwrap();
        assert_eq!(game.get_active_neighbors_count((0, 0, 0).into()), 1);
        assert_eq!(game.get_active_neighbors_count((1, 1, 0).into()), 5);
        assert_eq!(game.get_active_neighbors_count((1, 1, 1).into()), 5);
        assert_eq!(game.get_active_neighbors_count((2, 1, 1).into()), 4);
    }

    #[test]
    fn test_step() {
        let mut game = Conway3D::try_from(SAMPLE).unwrap();
        assert_eq!(game.get_bounds(), ((0, 0, 0).into(), (2, 2, 0).into()));
        game.step();
        assert_eq!(game.get_bounds(), ((-1, -1, -1).into(), (3, 3, 1).into()));

        assert_eq!(game.get_cell_at_position((0, 1, -1).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((2, 2, -1).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((1, 3, -1).into()), Cell::Active);
    }

    #[test]
    fn test_run_6_steps() {
        let mut game = Conway3D::try_from(SAMPLE).unwrap();
        game.run_steps(6);
        assert_eq!(game.count_active_cells(), 112);
    }
}
