//! # Day 13: Shuttle Search
//!
//! Your ferry can make it safely to a nearby port, but it won't get much further. When you call to book another ship, you discover that no ships embark from that port to your vacation island. You'll need to get from the port to the nearest airport.
//!
//! Fortunately, a shuttle bus service is available to bring you from the sea port to the airport! Each bus has an ID number that also indicates how often the bus leaves for the airport.
//!
//! Bus schedules are defined based on a timestamp that measures the number of minutes since some fixed reference point in the past. At timestamp 0, every bus simultaneously departed from the sea port. After that, each bus travels to the airport, then various other locations, and finally returns to the sea port to repeat its journey forever.
//!
//! The time this loop takes a particular bus is also its ID number: the bus with ID 5 departs from the sea port at timestamps 0, 5, 10, 15, and so on. The bus with ID 11 departs at 0, 11, 22, 33, and so on. If you are there when the bus departs, you can ride that bus to the airport!
//!
//! Your notes (your puzzle input) consist of two lines. The first line is your estimate of the earliest timestamp you could depart on a bus. The second line lists the bus IDs that are in service according to the shuttle company; entries that show x must be out of service, so you decide to ignore them.
//!
//! To save time once you arrive, your goal is to figure out the earliest bus you can take to the airport. (There will be exactly one such bus.)
//!
//! For example, suppose you have the following notes:
//!
//! ```text
//! 939
//! 7,13,x,x,59,x,31,19
//! ```
//!
//! Here, the earliest timestamp you could depart is 939, and the bus IDs in service are 7, 13, 59, 31, and 19. Near timestamp 939, these bus IDs depart at the times marked D:
//!
//! ```text
//! time   bus 7   bus 13  bus 59  bus 31  bus 19
//! 929      .       .       .       .       .
//! 930      .       .       .       D       .
//! 931      D       .       .       .       D
//! 932      .       .       .       .       .
//! 933      .       .       .       .       .
//! 934      .       .       .       .       .
//! 935      .       .       .       .       .
//! 936      .       D       .       .       .
//! 937      .       .       .       .       .
//! 938      D       .       .       .       .
//! 939      .       .       .       .       .
//! 940      .       .       .       .       .
//! 941      .       .       .       .       .
//! 942      .       .       .       .       .
//! 943      .       .       .       .       .
//! 944      .       .       D       .       .
//! 945      D       .       .       .       .
//! 946      .       .       .       .       .
//! 947      .       .       .       .       .
//! 948      .       .       .       .       .
//! 949      .       D       .       .       .
//! ```
//!
//! The earliest bus you could take is bus ID 59. It doesn't depart until timestamp 944, so you would need to wait 944 - 939 = 5 minutes before it departs. Multiplying the bus ID by the number of minutes you'd need to wait gives 295.
//!
//! What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus?
//!
//! # Part Two
//!
//! The shuttle company is running a contest: one gold coin for anyone that can find the earliest timestamp such that the first bus ID departs at that time and each subsequent listed bus ID departs at that subsequent minute. (The first line in your input is no longer relevant.)
//!
//! For example, suppose you have the same list of bus IDs as above:
//!
//! ```text
//! 7,13,x,x,59,x,31,19
//! ```
//!
//! An x in the schedule means there are no constraints on what bus IDs must depart at that time.
//!
//! This means you are looking for the earliest timestamp (called t) such that:
//!
//! - Bus ID 7 departs at timestamp t.
//! - Bus ID 13 departs one minute after timestamp t.
//! - There are no requirements or restrictions on departures at two or three minutes after timestamp t.
//! - Bus ID 59 departs four minutes after timestamp t.
//! - There are no requirements or restrictions on departures at five minutes after timestamp t.
//! - Bus ID 31 departs six minutes after timestamp t.
//! - Bus ID 19 departs seven minutes after timestamp t.
//!
//! The only bus departures that matter are the listed bus IDs at their specific offsets from t. Those bus IDs can depart at other times, and other bus IDs can depart at those times. For example, in the list above, because bus ID 19 must depart seven minutes after the timestamp at which bus ID 7 departs, bus ID 7 will always also be departing with bus ID 19 at seven minutes after timestamp t.
//!
//! In this example, the earliest timestamp at which this occurs is 1068781:
//!
//! ```text
//! time     bus 7   bus 13  bus 59  bus 31  bus 19
//! 1068773    .       .       .       .       .
//! 1068774    D       .       .       .       .
//! 1068775    .       .       .       .       .
//! 1068776    .       .       .       .       .
//! 1068777    .       .       .       .       .
//! 1068778    .       .       .       .       .
//! 1068779    .       .       .       .       .
//! 1068780    .       .       .       .       .
//! 1068781    D       .       .       .       .
//! 1068782    .       D       .       .       .
//! 1068783    .       .       .       .       .
//! 1068784    .       .       .       .       .
//! 1068785    .       .       D       .       .
//! 1068786    .       .       .       .       .
//! 1068787    .       .       .       D       .
//! 1068788    D       .       .       .       D
//! 1068789    .       .       .       .       .
//! 1068790    .       .       .       .       .
//! 1068791    .       .       .       .       .
//! 1068792    .       .       .       .       .
//! 1068793    .       .       .       .       .
//! 1068794    .       .       .       .       .
//! 1068795    D       D       .       .       .
//! 1068796    .       .       .       .       .
//! 1068797    .       .       .       .       .
//! ```
//!
//! In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes after t). This is fine; the only requirement on that minute is that bus ID 19 departs then, and it does.
//!
//! Here are some other examples:
//!
//! - The earliest timestamp that matches the list 17,x,13,19 is 3417.
//! - 67,7,59,61 first occurs at timestamp 754018.
//! - 67,x,7,59,61 first occurs at timestamp 779210.
//! - 67,7,x,59,61 first occurs at timestamp 1261476.
//! - 1789,37,47,1889 first occurs at timestamp 1202161486.
//!
//! However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger than 100000000000000!
//!
//! What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    let (target, schedule) = extract_schedules(INPUT_VALUES);
    let scheduler = Scheduler::from_input(schedule);
    let (wait_time, bus_id) = scheduler.scan_buses_for_target_time(target);

    wait_time * bus_id
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let (_, schedule) = extract_schedules(INPUT_VALUES);
    Scheduler::from_input(schedule).compute_successive_departures_time()
}

/// Extract schedules from input string.
///
/// # Arguments
///
/// * `input` - Input string
pub fn extract_schedules(input: &str) -> (usize, &str) {
    let mut lines = input.lines();
    (
        lines.next().and_then(|x| x.parse::<usize>().ok()).unwrap(),
        lines.next().unwrap(),
    )
}

/// Bus scheduler
pub struct Scheduler {
    data: Vec<Option<usize>>,
}

impl Scheduler {
    /// Creates new scheduler from input.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn from_input(input: &str) -> Self {
        Self {
            data: input.trim().split(',').map(|x| x.parse().ok()).collect(),
        }
    }

    /// Creates new scheduler from vec.
    ///
    /// # Arguments
    ///
    /// * `data` - Vec
    pub fn from_vec(data: Vec<Option<usize>>) -> Self {
        Self { data }
    }

    /// Scan bus for target time.
    /// Returns a `(wait_time, bus_id)` tuple.
    ///
    /// # Arguments
    ///
    /// * `target` - Target time
    pub fn scan_buses_for_target_time(&self, target: usize) -> (usize, usize) {
        self.data
            .iter()
            .filter_map(|b| b.and_then(|b| Some((b - target.rem_euclid(b), b))))
            .min()
            .unwrap()
    }

    /// Compute successive departures time.
    pub fn compute_successive_departures_time(&self) -> usize {
        let nums = self.get_bus_numbers();
        let offsets = self.get_offsets();

        let mut t = 0;
        let mut last_lcm = 1;
        let mut found = false;

        while !found {
            found = true;

            for idx in 1..nums.len() {
                let n_a = nums[idx - 1];
                let n_b = nums[idx];
                let offset_a = offsets[idx - 1];
                let offset_b = offsets[idx];

                let (nt, _, _) = Self::calculate_x(n_a, n_b, offset_a, offset_b, t, last_lcm);

                t = nt;
                last_lcm = nums[0..idx].iter().product();

                if !Self::validate_t(&nums[0..idx], &offsets[0..idx], t) {
                    // Try again
                    found = false;
                    break;
                }
            }
        }

        t
    }

    const fn calculate_x(
        n_a: usize,
        n_b: usize,
        offset_a: usize,
        offset_b: usize,
        t_initial: usize,
        t_increment: usize,
    ) -> (usize, usize, usize) {
        let mut t = t_initial;
        let mut x;
        let mut y;

        loop {
            x = (t + offset_a) / n_a;
            y = (t + offset_b) / n_b;

            // Try to validate equations
            if y * n_b == t + offset_b && x * n_a == t + offset_a {
                return (t, x, y);
            }

            t += t_increment;
        }
    }

    fn validate_t(n: &[usize], a: &[usize], t: usize) -> bool {
        for i in 0..n.len() {
            if (t + a[i]) % n[i] != 0 {
                return false;
            }
        }

        true
    }

    fn get_offsets(&self) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, n)| n.map(|_| idx))
            .collect()
    }

    fn get_bus_numbers(&self) -> Vec<usize> {
        self.data.iter().filter_map(|n| *n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 136;
    const EX2_OUTPUT: usize = 305_068_317_272_992;

    const SAMPLE: &str = r#"939
    7,13,x,x,59,x,31,19"#;

    #[test]
    fn test_scheduler_parse() {
        let (_, schedule_line) = extract_schedules(SAMPLE);
        let scheduler = Scheduler::from_input(schedule_line);
        assert_eq!(scheduler.get_bus_numbers(), vec![7, 13, 59, 31, 19]);
    }

    #[test]
    fn test_scan_buses_for_target_time() {
        let (target, schedule_line) = extract_schedules(SAMPLE);
        let scheduler = Scheduler::from_input(schedule_line);
        let result = scheduler.scan_buses_for_target_time(target);

        assert_eq!(result, (5, 59));
    }

    #[test]
    fn test_compute_successive_departures_time() {
        assert_eq!(
            Scheduler::from_vec(vec![Some(17), None, Some(13), Some(19)])
                .compute_successive_departures_time(),
            3417
        );
        assert_eq!(
            Scheduler::from_vec(vec![Some(67), Some(7), Some(59), Some(61)])
                .compute_successive_departures_time(),
            754_018
        );
        assert_eq!(
            Scheduler::from_vec(vec![Some(67), None, Some(7), Some(59), Some(61)])
                .compute_successive_departures_time(),
            779_210
        );
        assert_eq!(
            Scheduler::from_vec(vec![Some(67), Some(7), None, Some(59), Some(61)])
                .compute_successive_departures_time(),
            1_261_476
        );
        assert_eq!(
            Scheduler::from_vec(vec![Some(1789), Some(37), Some(47), Some(1889)])
                .compute_successive_departures_time(),
            1_202_161_486
        );
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
