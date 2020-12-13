//! # Day 11: Seating System
//!
//! Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!
//!
//! By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).
//!
//! The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:
//!
//! ```text
//! L.LL.LL.LL
//! LLLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLLL
//! L.LLLLLL.L
//! L.LLLLL.LL
//! ```
//!
//! Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:
//!
//! - If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
//! - If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
//! - Otherwise, the seat's state does not change.
//!
//! Floor (.) never changes; seats don't move, and nobody sits on the floor.
//!
//! After one round of these rules, every seat in the example layout becomes occupied:
//!
//! ```text
//! #.##.##.##
//! #######.##
//! #.#.#..#..
//! ####.##.##
//! #.##.##.##
//! #.#####.##
//! ..#.#.....
//! ##########
//! #.######.#
//! #.#####.##
//! ```
//!
//! After a second round, the seats with four or more occupied adjacent seats become empty again:
//!
//! ```text
//! #.LL.L#.##
//! #LLLLLL.L#
//! L.L.L..L..
//! #LLL.LL.L#
//! #.LL.LL.LL
//! #.LLLL#.##
//! ..L.L.....
//! #LLLLLLLL#
//! #.LLLLLL.L
//! #.#LLLL.##
//! ```
//!
//! This process continues for three more rounds:
//!
//! ```text
//! #.##.L#.##
//! #L###LL.L#
//! L.#.#..#..
//! #L##.##.L#
//! #.##.LL.LL
//! #.###L#.##
//! ..#.#.....
//! #L######L#
//! #.LL###L.L
//! #.#L###.##
//! ```
//!
//! ```text
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.L.L..#..
//! #LLL.##.L#
//! #.LL.LL.LL
//! #.LL#L#.##
//! ..L.L.....
//! #L#LLLL#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//! ```
//!
//! ```text
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.#.L..#..
//! #L##.##.L#
//! #.#L.LL.LL
//! #.#L#L#.##
//! ..L.L.....
//! #L#L##L#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//! ```
//!
//! At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.
//!
//! Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?
//!
//! # Part Two
//!
//! As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats - they care about the first seat they can see in each of those eight directions!
//!
//! Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of those eight directions. For example, the empty seat below would see eight occupied seats:
//!
//! ```text
//! .......#.
//! ...#.....
//! .#.......
//! .........
//! ..#L....#
//! ....#....
//! .........
//! #........
//! ...#.....
//! ```
//!
//! The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:
//!
//! ```text
//! .............
//! .L.L.#.#.#.#.
//! .............
//! ```
//!
//! The empty seat below would see no occupied seats:
//!
//! ```text
//! .##.##.
//! #.#.#.#
//! ##...##
//! ...L...
//! ##...##
//! #.#.#.#
//! .##.##.
//! ```
//!
//! Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.
//!
//! Given the same starting layout as above, these new rules cause the seating area to shift around as follows:
//!
//! ```text
//! L.LL.LL.LL
//! LLLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLLL
//! L.LLLLLL.L
//! L.LLLLL.LL
//! ```
//!
//! ```text
//! #.##.##.##
//! #######.##
//! #.#.#..#..
//! ####.##.##
//! #.##.##.##
//! #.#####.##
//! ..#.#.....
//! ##########
//! #.######.#
//! #.#####.##
//! ```
//!
//! ```text
//! #.LL.LL.L#
//! #LLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLL#
//! #.LLLLLL.L
//! #.LLLLL.L#
//! ```
//!
//! ```text
//! #.L#.##.L#
//! #L#####.LL
//! L.#.#..#..
//! ##L#.##.##
//! #.##.#L.##
//! #.#####.#L
//! ..#.#.....
//! LLL####LL#
//! #.L#####.L
//! #.L####.L#
//! ```
//!
//! ```text
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##LL.LL.L#
//! L.LL.LL.L#
//! #.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! ```text
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##L#.#L.L#
//! L.L#.#L.L#
//! #.L####.LL
//! ..#.#.....
//! LLL###LLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! ```text
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##L#.#L.L#
//! L.L#.LL.L#
//! #.LLLL#.LL
//! ..#.L.....
//! LLL###LLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.
//!
//! Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?

#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]

use eyre::{eyre, Result};

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    SeatLayout::from_input(INPUT_VALUES)
        .expect("bad input")
        .run_until_stable()
        .occupied_seats
}

/// Part two answer.
pub fn run_ex2() -> usize {
    SeatLayout::from_input(INPUT_VALUES)
        .expect("bad input")
        .run_with_visibility_until_stable()
        .occupied_seats
}

/// Seat state
#[derive(Debug, Copy, Clone)]
pub enum SeatState {
    /// No seat
    Floor,
    /// Free seat
    Free,
    /// Occupied seat
    Occupied,
}

impl SeatState {
    /// Creates seat state from character.
    ///
    /// # Arguments
    ///
    /// * `character` - Character
    ///
    /// # Errors
    ///
    /// * Bad character
    pub fn from_char(character: char) -> Result<Self> {
        match character {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::Free),
            '#' => Ok(Self::Occupied),
            e => Err(eyre!("Bad seat state character: {}", e)),
        }
    }

    /// Convert state to character.
    pub const fn to_char(self) -> char {
        match self {
            Self::Floor => '.',
            Self::Free => 'L',
            Self::Occupied => '#',
        }
    }
}

/// Seat layout stats
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SeatLayoutStats {
    /// Total seats
    pub total_seats: usize,
    /// Free seats
    pub free_seats: usize,
    /// Occupied seats
    pub occupied_seats: usize,
}

impl SeatLayoutStats {
    /// Creates empty stats.
    pub const fn new_empty() -> Self {
        Self {
            total_seats: 0,
            free_seats: 0,
            occupied_seats: 0,
        }
    }
}

/// Seat layout
#[derive(Debug)]
pub struct SeatLayout {
    frontbuffer: Vec<Vec<SeatState>>,
    backbuffer: Vec<Vec<SeatState>>,
}

impl SeatLayout {
    /// Creates seat layout from input string.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    ///
    /// # Errors
    ///
    /// * Bad format
    pub fn from_input(input: &str) -> Result<Self> {
        let data = input
            .lines()
            .map(|x| {
                x.trim()
                    .chars()
                    .map(SeatState::from_char)
                    .collect::<Result<Vec<SeatState>>>()
            })
            .collect::<Result<Vec<Vec<SeatState>>>>()?;

        Ok(Self {
            frontbuffer: data.clone(),
            backbuffer: data,
        })
    }

    /// Get layout size
    pub fn get_size(&self) -> (usize, usize) {
        (self.frontbuffer[0].len(), self.frontbuffer.len())
    }

    /// Update state using neighbors.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    /// * `old_state` - Old seat state
    fn update_state_from_neighbors(&self, x: usize, y: usize, old_state: SeatState) -> SeatState {
        match old_state {
            SeatState::Floor => old_state,
            SeatState::Free => {
                if self.count_neighbors_occupied_seats(x, y) == 0 {
                    SeatState::Occupied
                } else {
                    old_state
                }
            }
            SeatState::Occupied => {
                if self.count_neighbors_occupied_seats(x, y) >= 4 {
                    SeatState::Free
                } else {
                    old_state
                }
            }
        }
    }

    /// Update state using visibility.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    /// * `old_state` - Old seat state
    pub fn update_state_from_visibility(
        &self,
        x: usize,
        y: usize,
        old_state: SeatState,
    ) -> SeatState {
        match old_state {
            SeatState::Floor => old_state,
            SeatState::Free => {
                if self.count_visible_occupied_seats(x, y) == 0 {
                    SeatState::Occupied
                } else {
                    old_state
                }
            }
            SeatState::Occupied => {
                if self.count_visible_occupied_seats(x, y) >= 5 {
                    SeatState::Free
                } else {
                    old_state
                }
            }
        }
    }

    /// Swap front and back buffers.
    pub fn swap_buffers(&mut self) {
        let (w, h) = self.get_size();

        for j in 0..h {
            for i in 0..w {
                self.frontbuffer[j][i] = self.backbuffer[j][i];
            }
        }
    }

    /// Step simulation.
    pub fn step(&mut self) -> SeatLayoutStats {
        let mut total_seats = 0;
        let mut free_seats = 0;
        let mut occupied_seats = 0;

        let (w, h) = self.get_size();

        for j in 0..h {
            for i in 0..w {
                let old_state = self.frontbuffer[j][i];
                let new_state = self.update_state_from_neighbors(i, j, old_state);
                self.backbuffer[j][i] = new_state;

                match new_state {
                    SeatState::Floor => (),
                    SeatState::Free => {
                        total_seats += 1;
                        free_seats += 1;
                    }
                    SeatState::Occupied => {
                        total_seats += 1;
                        occupied_seats += 1;
                    }
                }
            }
        }

        self.swap_buffers();

        SeatLayoutStats {
            total_seats,
            free_seats,
            occupied_seats,
        }
    }

    /// Step simulation with visibility check.
    pub fn step_with_visibility(&mut self) -> SeatLayoutStats {
        let mut total_seats = 0;
        let mut free_seats = 0;
        let mut occupied_seats = 0;

        let (w, h) = self.get_size();

        for j in 0..h {
            for i in 0..w {
                let old_state = self.frontbuffer[j][i];
                let new_state = self.update_state_from_visibility(i, j, old_state);
                self.backbuffer[j][i] = new_state;

                match new_state {
                    SeatState::Floor => (),
                    SeatState::Free => {
                        total_seats += 1;
                        free_seats += 1;
                    }
                    SeatState::Occupied => {
                        total_seats += 1;
                        occupied_seats += 1;
                    }
                }
            }
        }

        self.swap_buffers();

        SeatLayoutStats {
            total_seats,
            free_seats,
            occupied_seats,
        }
    }

    /// Run steps until the simulation is stable.
    pub fn run_until_stable(&mut self) -> SeatLayoutStats {
        let mut last_stats = SeatLayoutStats::new_empty();

        loop {
            let new_stats = self.step();
            if new_stats == last_stats {
                return new_stats;
            } else {
                last_stats = new_stats;
            }
        }
    }

    /// Run steps with visibility until the simulation is stable.
    pub fn run_with_visibility_until_stable(&mut self) -> SeatLayoutStats {
        let mut last_stats = SeatLayoutStats::new_empty();

        loop {
            let new_stats = self.step_with_visibility();
            if new_stats == last_stats {
                return new_stats;
            } else {
                last_stats = new_stats;
            }
        }
    }

    /// Count neighbors occupied seats for position.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    pub fn count_neighbors_occupied_seats(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    let (nx, ny) = ((x as isize + i) as usize, (y as isize + j) as usize);
                    if let Some(SeatState::Occupied) = self.get_seat_state_at_position(nx, ny) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Count visible occupied seats for position.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    pub fn count_visible_occupied_seats(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    let (mut nx, mut ny) = ((x as isize + i) as usize, (y as isize + j) as usize);
                    loop {
                        match self.get_seat_state_at_position(nx, ny) {
                            Some(SeatState::Occupied) => {
                                // Stop scan and count
                                count += 1;
                                break;
                            }
                            Some(SeatState::Floor) => {
                                // Continue to scan
                            }
                            None | Some(SeatState::Free) => {
                                // Stop scan
                                break;
                            }
                        }

                        nx = (nx as isize + i) as usize;
                        ny = (ny as isize + j) as usize;
                    }
                }
            }
        }

        count
    }

    /// Get seat state at position.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    pub fn get_seat_state_at_position(&self, x: usize, y: usize) -> Option<SeatState> {
        let (w, h) = self.get_size();
        if x < w && y < h {
            Some(self.frontbuffer[y][x])
        } else {
            None
        }
    }

    /// Write layout to string
    pub fn write_to_string(&self) -> String {
        let (w, h) = self.get_size();

        let mut output = String::new();

        for j in 0..h {
            for i in 0..w {
                let c = self.frontbuffer[j][i];
                output.push(c.to_char());
            }
            output.push('\n');
        }

        output
    }

    /// Show layout
    pub fn show(&self) {
        println!("{}", self.write_to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 2344;
    const EX2_OUTPUT: usize = 2076;

    const SAMPLE_LAYOUT: &str = r###"L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL"###;

    const SAMPLE_SCAN_1: &str = r###".......#.
    ...#.....
    .#.......
    .........
    ..#L....#
    ....#....
    .........
    #........
    ...#....."###;

    const SAMPLE_SCAN_2: &str = r###".............
    .L.L.#.#.#.#.
    ............."###;

    const SAMPLE_SCAN_3: &str = r###".##.##.
    #.#.#.#
    ##...##
    ...L...
    ##...##
    #.#.#.#
    .##.##."###;

    #[test]
    fn test_layout_parse() {
        let layout = SeatLayout::from_input(SAMPLE_LAYOUT).unwrap();
        assert_eq!(layout.get_size(), (10, 10));
    }

    #[test]
    fn test_step() {
        let mut layout = SeatLayout::from_input(SAMPLE_LAYOUT).unwrap();

        let stats = layout.step();
        assert_eq!(stats.free_seats, 0);
        assert_eq!(stats.occupied_seats, stats.total_seats);
    }

    #[test]
    fn test_run_until_stable() {
        let mut layout = SeatLayout::from_input(SAMPLE_LAYOUT).unwrap();
        let stats = layout.run_until_stable();

        assert_eq!(stats.occupied_seats, 37);
    }

    #[test]
    fn test_run_with_visibility_until_stable() {
        let mut layout = SeatLayout::from_input(SAMPLE_LAYOUT).unwrap();
        let stats = layout.run_with_visibility_until_stable();

        assert_eq!(stats.occupied_seats, 26);
    }

    #[test]
    fn test_scan_1() {
        let layout = SeatLayout::from_input(SAMPLE_SCAN_1).unwrap();
        assert_eq!(layout.count_visible_occupied_seats(3, 4), 8);
    }

    #[test]
    fn test_scan_2() {
        let layout = SeatLayout::from_input(SAMPLE_SCAN_2).unwrap();
        assert_eq!(layout.count_visible_occupied_seats(1, 1), 0);
        assert_eq!(layout.count_visible_occupied_seats(3, 1), 1);
    }

    #[test]
    fn test_scan_3() {
        let layout = SeatLayout::from_input(SAMPLE_SCAN_3).unwrap();
        assert_eq!(layout.count_visible_occupied_seats(3, 3), 0);
    }

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }

    #[test]
    fn test_run_ex2() {
        assert_eq!(run_ex2(), EX2_OUTPUT);
    }
}
