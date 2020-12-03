//! Advent of Code 2020

#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::pub_enum_variant_names,
    clippy::must_use_candidate
)]

pub mod days;
mod shell;

pub use shell::initialize_command_line;
