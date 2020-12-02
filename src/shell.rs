//! Shell module

use eyre::Result;
use structopt::StructOpt;

use super::days;

#[derive(Debug, StructOpt)]
enum Days {
    Day01,
    Day02,
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

pub fn initialize_command_line() -> Result<()> {
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
        },
    }

    Ok(())
}
