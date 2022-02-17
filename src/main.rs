#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    rust_2021_compatibility,
    clippy::all,
    clippy::pedantic
)]
#![allow(
    dead_code,
    clippy::missing_errors_doc,
    clippy::similar_names,
    clippy::cast_possible_truncation
)]

use std::{env, process};
use tic_tac_toe::cli::ParsedArgs;
use tic_tac_toe::run;

mod exit_code {
    pub const SUCCESS: i32 = 0;
    pub const FAILURE: i32 = 1;
    pub const INVALID_ARGS: i32 = 2;
}

fn main() {
    let parsed_args = ParsedArgs::from_iterator(env::args_os()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(exit_code::INVALID_ARGS);
    });
    if let Err(e) = run(parsed_args) {
        eprintln!("{}", e);
        process::exit(exit_code::FAILURE);
    } else {
        process::exit(exit_code::SUCCESS);
    }
}
