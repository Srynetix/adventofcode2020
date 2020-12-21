//! Shell module

use eyre::{eyre, Result};
use structopt::StructOpt;

use super::days;

#[derive(Debug, StructOpt)]
enum Command {
    /// Run one specific day
    Run {
        /// Day
        day: usize,
    },
    /// Run all days
    RunAll,
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

fn run_day<F1, F2, O1, O2>(day: usize, ex1: F1, ex2: F2) -> Result<()>
where
    F1: Fn() -> O1,
    F2: Fn() -> O2,
    O1: std::fmt::Display,
    O2: std::fmt::Display,
{
    println!("Day {:<2} > [Ex1] {:<16} | [Ex2] {:<16}", day, ex1(), ex2());
    Ok(())
}

fn run_day_wrapper(d: usize) -> Result<()> {
    match d {
        1 => run_day(d, days::day01::run_ex1, days::day01::run_ex2),
        2 => run_day(d, days::day02::run_ex1, days::day02::run_ex2),
        3 => run_day(d, days::day03::run_ex1, days::day03::run_ex2),
        4 => run_day(d, days::day04::run_ex1, days::day04::run_ex2),
        5 => run_day(d, days::day05::run_ex1, days::day05::run_ex2),
        6 => run_day(d, days::day06::run_ex1, days::day06::run_ex2),
        7 => run_day(d, days::day07::run_ex1, days::day07::run_ex2),
        8 => run_day(d, days::day08::run_ex1, days::day08::run_ex2),
        9 => run_day(d, days::day09::run_ex1, days::day09::run_ex2),
        10 => run_day(d, days::day10::run_ex1, days::day10::run_ex2),
        11 => run_day(d, days::day11::run_ex1, days::day11::run_ex2),
        12 => run_day(d, days::day12::run_ex1, days::day12::run_ex2),
        13 => run_day(d, days::day13::run_ex1, days::day13::run_ex2),
        14 => run_day(d, days::day14::run_ex1, days::day14::run_ex2),
        15 => run_day(d, days::day15::run_ex1, days::day15::run_ex2),
        16 => run_day(d, days::day16::run_ex1, days::day16::run_ex2),
        17 => run_day(d, days::day17::run_ex1, days::day17::run_ex2),
        18 => run_day(d, days::day18::run_ex1, days::day18::run_ex2),
        19 => run_day(d, days::day19::run_ex1, days::day19::run_ex2),
        20 => run_day(d, days::day20::run_ex1, days::day20::run_ex2),
        21 => run_day(d, days::day21::run_ex1, days::day21::run_ex2),
        d if d <= 25 => Err(eyre!("Day {} not implemented yet.", d)),
        d => Err(eyre!(
            "Day {} is not in Advent of Code day range (1-25).",
            d
        )),
    }
}

/// Initialize command line arguments.
pub fn initialize_command_line() {
    env_logger::init();

    let args = Opt::from_args();

    match args.cmd {
        Command::Run { day } => {
            if let Err(e) = run_day_wrapper(day) {
                eprintln!("Error: {}", e);
            }
        }
        Command::RunAll => {
            let mut day_idx = 1;
            while run_day_wrapper(day_idx).is_ok() {
                day_idx += 1;
            }
        }
    }
}
