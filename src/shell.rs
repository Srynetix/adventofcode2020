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

fn run_day<F1, F2>(day: usize, ex1: F1, ex2: F2) -> Result<()>
where
    F1: Fn() -> usize,
    F2: Fn() -> usize,
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
        _ => Err(eyre!("Unknown day: {}", d)),
    }
}

/// Initialize command line arguments.
pub fn initialize_command_line() {
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
