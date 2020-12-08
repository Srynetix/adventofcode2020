//! Shell module

use structopt::StructOpt;

use super::days;

#[derive(Debug, StructOpt)]
enum Days {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
}

#[derive(Debug, StructOpt)]
enum Command {
    Run {
        #[structopt(subcommand)]
        day: Days,
    },
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

/// Initialize command line arguments.
pub fn initialize_command_line() {
    let args = Opt::from_args();

    match args.cmd {
        Command::Run { day } => match day {
            Days::Day01 => {
                println!(
                    "Day 01:\n  - Ex1: {}\n  - Ex2: {}",
                    days::day01::run_ex1(),
                    days::day01::run_ex2()
                );
            }
            Days::Day02 => {
                println!(
                    "Day 02:\n  - Ex1: {}\n  - Ex2: {}",
                    days::day02::run_ex1(),
                    days::day02::run_ex2()
                )
            }
            Days::Day03 => {
                println!(
                    "Day 03:\n  - Ex1: {}\n  - Ex2: {}",
                    days::day03::run_ex1(),
                    days::day03::run_ex2()
                )
            }
            Days::Day04 => {
                println!(
                    "Day 04:\n  - Ex1: {}\n  - Ex2: {}",
                    days::day04::run_ex1(),
                    days::day04::run_ex2()
                )
            }
            Days::Day05 => {
                println!(
                    "Day 05:\n  - Ex1: {}\n  - Ex2: {}",
                    days::day05::run_ex1(),
                    days::day05::run_ex2()
                )
            }
            Days::Day06 => {
                println!(
                    "Day 06:\n  - Ex1: {}\n  - Ex2: {}",
                    days::day06::run_ex1(),
                    days::day06::run_ex2()
                )
            }
            Days::Day07 => {
                println!(
                    "Day 07:\n  - Ex1: {}\n  - Ex2: {}",
                    days::day07::run_ex1(),
                    days::day07::run_ex2()
                )
            }
            Days::Day08 => {
                println!(
                    "Day 08:\n  - Ex1: {}\n  - Ex2: {}",
                    days::day08::run_ex1(),
                    days::day08::run_ex2()
                )
            }
        },
    }
}
