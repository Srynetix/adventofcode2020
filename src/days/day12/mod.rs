//! # Day 12: Rain Risk
//!
//! Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!
//!
//! Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.
//!
//! The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:
//!
//! - Action N means to move north by the given value.
//! - Action S means to move south by the given value.
//! - Action E means to move east by the given value.
//! - Action W means to move west by the given value.
//! - Action L means to turn left the given number of degrees.
//! - Action R means to turn right the given number of degrees.
//! - Action F means to move forward by the given value in the direction the ship is currently facing.
//!
//! The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)
//!
//! For example:
//!
//! ```text
//! F10
//! N3
//! F7
//! R90
//! F11
//! ```
//!
//! These instructions would be handled as follows:
//!
//! - F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
//! - N3 would move the ship 3 units north to east 10, north 3.
//! - F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
//! - R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
//! - F11 would move the ship 11 units south to east 17, south 8.
//!
//! At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.
//!
//! Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?
//!
//! # Part Two
//!
//! Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.
//!
//! Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:
//!
//! - Action N means to move the waypoint north by the given value.
//! - Action S means to move the waypoint south by the given value.
//! - Action E means to move the waypoint east by the given value.
//! - Action W means to move the waypoint west by the given value.
//! - Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
//! - Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
//! - Action F means to move forward to the waypoint a number of times equal to the given value.
//!
//! The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.
//!
//! For example, using the same instructions as above:
//!
//! - F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
//! - N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
//! - F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
//! - R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
//! - F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
//!
//! After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.
//!
//! Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    compute_manhattan_distance(
        Ship::new().parse_and_execute_input_commands_no_waypoint(INPUT_VALUES),
    )
}

/// Part two answer.
pub fn run_ex2() -> usize {
    compute_manhattan_distance(Ship::new().parse_and_execute_input_commands_waypoint(INPUT_VALUES))
}

/// Compute Manhattan distance from a isize tuple.
///
/// # Arguments
///
/// * `(x, y)`: isize tuple
#[allow(clippy::cast_sign_loss)]
pub fn compute_manhattan_distance((x, y): (isize, isize)) -> usize {
    (x.abs() + y.abs()) as usize
}

/// Rotate position from `amount`.
///
/// # Arguments
///
/// * `(x, y)`: `isize` position tuple
/// * `amount`: Rotation amount
pub fn rotate_position((x, y): (isize, isize), amount: isize) -> (isize, isize) {
    let amount: f64 = amount as f64;
    let amount = amount.to_radians();
    let (cs, sn) = (amount.cos(), amount.sin());

    let (fx, fy) = (x as f64, y as f64);
    let (nx, ny) = (fx * cs - fy * sn, fx.mul_add(sn, fy * cs));

    (nx.round() as isize, ny.round() as isize)
}

/// Command direction
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CommandDirection {
    /// North
    North(isize),
    /// South
    South(isize),
    /// East
    East(isize),
    /// West
    West(isize),
    /// Forward
    Forward(isize),
    /// Left
    Left(isize),
    /// Right
    Right(isize),
}

impl CommandDirection {
    /// Extract command direction and value from input.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn from_input(input: &str) -> Self {
        let mut chars = input.chars();
        let letter = chars.next().unwrap();
        let value = chars.collect::<String>().parse::<isize>().unwrap();

        match letter {
            'F' => Self::Forward(value),
            'N' => Self::North(value),
            'S' => Self::South(value),
            'E' => Self::East(value),
            'W' => Self::West(value),
            'L' => Self::Left(value),
            'R' => Self::Right(value),
            l => panic!("Unknown direction: {}", l),
        }
    }
}

/// Ship
///
/// Initial direction, `0` degrees:
/// ```text
///    N
/// W  >  E
///    S
/// ```
///
/// Initial waypoint is at `(10, 1)`
#[derive(Debug, Default)]
pub struct Ship {
    rotation: isize,
    x: isize,
    y: isize,
    waypoint_rel_x: isize,
    waypoint_rel_y: isize,
}

impl Ship {
    /// Creates a new ship.
    pub const fn new() -> Self {
        Self {
            rotation: 0,
            x: 0,
            y: 0,
            waypoint_rel_x: 10,
            waypoint_rel_y: 1,
        }
    }

    /// Execute command without waypoint.
    ///
    /// # Arguments
    ///
    /// * `command` - Command
    pub fn execute_command_no_waypoint(&mut self, command: CommandDirection) -> (isize, isize) {
        match command {
            CommandDirection::North(v) => self.y += v,
            CommandDirection::East(v) => self.x += v,
            CommandDirection::South(v) => self.y -= v,
            CommandDirection::West(v) => self.x -= v,
            CommandDirection::Left(v) => self.rotate_ship_direction(v),
            CommandDirection::Right(v) => self.rotate_ship_direction(-v),
            CommandDirection::Forward(v) => self.move_forward(v),
        }

        (self.x, self.y)
    }

    /// Execute command with waypoint.
    ///
    /// # Arguments
    ///
    /// * `command` - Command
    pub fn execute_command_waypoint(&mut self, command: CommandDirection) -> (isize, isize) {
        match command {
            CommandDirection::North(v) => self.waypoint_rel_y += v,
            CommandDirection::East(v) => self.waypoint_rel_x += v,
            CommandDirection::South(v) => self.waypoint_rel_y -= v,
            CommandDirection::West(v) => self.waypoint_rel_x -= v,
            CommandDirection::Left(v) => self.rotate_waypoint(v),
            CommandDirection::Right(v) => self.rotate_waypoint(-v),
            CommandDirection::Forward(v) => self.move_towards_waypoint(v),
        }

        (self.x, self.y)
    }

    /// Rotate waypoint.
    ///
    /// # Arguments
    ///
    /// * `amount` - Rotation amount
    pub fn rotate_waypoint(&mut self, amount: isize) {
        let (x, y) = rotate_position((self.waypoint_rel_x, self.waypoint_rel_y), amount);
        self.waypoint_rel_x = x;
        self.waypoint_rel_y = y;
    }

    /// Rotate ship direction.
    ///
    /// # Arguments
    ///
    /// * `amount` - Rotation amount
    pub fn rotate_ship_direction(&mut self, amount: isize) {
        self.rotation = (self.rotation + amount).rem_euclid(360);
    }

    /// Move ship forward.
    ///
    /// # Arguments
    ///
    /// * `amount` - Movement factor
    pub fn move_forward(&mut self, amount: isize) {
        match self.rotation {
            // East
            0 => self.x += amount,
            // North
            90 => self.y += amount,
            // West
            180 => self.x -= amount,
            // South
            270 => self.y -= amount,
            // Unsupported
            u => panic!("Unsupported rotation: {}", u),
        }
    }

    /// Move ship towards waypoint.
    ///
    /// # Arguments
    ///
    /// * `amount` - Movement factor
    pub fn move_towards_waypoint(&mut self, amount: isize) {
        self.x += self.waypoint_rel_x * amount;
        self.y += self.waypoint_rel_y * amount;
    }

    /// Parse and execute input commands without waypoint.
    /// Returns last position after command execution.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_and_execute_input_commands_no_waypoint(&mut self, input: &str) -> (isize, isize) {
        let commands: Vec<_> = input
            .lines()
            .map(|l| CommandDirection::from_input(l.trim()))
            .collect();
        for command in commands {
            self.execute_command_no_waypoint(command);
        }

        (self.x, self.y)
    }

    /// Parse and execute input commands with waypoint.
    /// Returns last position after command execution.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_and_execute_input_commands_waypoint(&mut self, input: &str) -> (isize, isize) {
        let commands: Vec<_> = input
            .lines()
            .map(|l| CommandDirection::from_input(l.trim()))
            .collect();
        for command in commands {
            self.execute_command_waypoint(command);
        }

        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 998;
    const EX2_OUTPUT: usize = 71586;

    const SAMPLE: &str = r#"F10
    N3
    F7
    R90
    F11"#;

    #[test]
    fn test_parse_command() {
        assert_eq!(
            CommandDirection::from_input("F180"),
            CommandDirection::Forward(180)
        );
        assert_eq!(
            CommandDirection::from_input("L90"),
            CommandDirection::Left(90)
        );
    }

    #[test]
    fn test_sample_execution_steps_no_waypoint() {
        let mut ship = Ship::new();
        let lines: Vec<_> = SAMPLE.lines().map(str::trim).collect();

        assert_eq!(
            ship.execute_command_no_waypoint(CommandDirection::from_input(lines[0])),
            (10, 0)
        );
        assert_eq!(
            ship.execute_command_no_waypoint(CommandDirection::from_input(lines[1])),
            (10, 3)
        );
        assert_eq!(
            ship.execute_command_no_waypoint(CommandDirection::from_input(lines[2])),
            (17, 3)
        );
        assert_eq!(
            ship.execute_command_no_waypoint(CommandDirection::from_input(lines[3])),
            (17, 3)
        );
        assert_eq!(
            ship.execute_command_no_waypoint(CommandDirection::from_input(lines[4])),
            (17, -8)
        );
    }

    #[test]
    fn test_sample_execution_no_waypoint() {
        let mut ship = Ship::new();
        let out = ship.parse_and_execute_input_commands_no_waypoint(SAMPLE);
        assert_eq!(out, (17, -8));
        assert_eq!(compute_manhattan_distance(out), 25);
    }

    #[test]
    fn test_rotate_position() {
        assert_eq!(rotate_position((10, 1), 0), (10, 1));
        assert_eq!(rotate_position((10, 1), 90), (-1, 10));
        assert_eq!(rotate_position((10, 1), 180), (-10, -1));
        assert_eq!(rotate_position((10, 1), 270), (1, -10));
        assert_eq!(rotate_position((10, 1), 360), (10, 1));
    }

    #[test]
    fn test_sample_execution_steps_waypoint() {
        let mut ship = Ship::new();
        let lines: Vec<_> = SAMPLE.lines().map(str::trim).collect();

        assert_eq!(
            ship.execute_command_waypoint(CommandDirection::from_input(lines[0])),
            (100, 10)
        );
        assert_eq!(
            ship.execute_command_waypoint(CommandDirection::from_input(lines[1])),
            (100, 10)
        );
        assert_eq!(
            ship.execute_command_waypoint(CommandDirection::from_input(lines[2])),
            (170, 38)
        );
        assert_eq!(
            ship.execute_command_waypoint(CommandDirection::from_input(lines[3])),
            (170, 38)
        );
        assert_eq!(
            ship.execute_command_waypoint(CommandDirection::from_input(lines[4])),
            (214, -72)
        );
    }

    #[test]
    fn test_sample_execution_waypoint() {
        let mut ship = Ship::new();
        let out = ship.parse_and_execute_input_commands_waypoint(SAMPLE);
        assert_eq!(out, (214, -72));
        assert_eq!(compute_manhattan_distance(out), 286);
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
