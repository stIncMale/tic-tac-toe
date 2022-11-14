#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    unused_qualifications,
    clippy::all,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::alloc_instead_of_core,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags
)]
#![allow(
    clippy::similar_names,
    clippy::cast_possible_truncation,
    // uncomment below to simplify editing, comment out again before committing
    // clippy::pedantic,
    // unused_imports,
    // unused_variables,
    // unused_mut,
    // unreachable_code,
    // dead_code,
)]

use std::{env, process::ExitCode};

use clap::error::ErrorKind::{DisplayHelp, DisplayVersion};
use tic_tac_toe_lib::{cli::ParsedArgs, run};

/// Exit codes complementing the canonical ones in [`ExitCode`](std::process::ExitCode).
mod exit_code {
    pub const INVALID_ARGS: u8 = 2;
}

fn main() -> ExitCode {
    match ParsedArgs::from_iterator(env::args_os()) {
        Ok(parsed_args) => {
            if let Err(e) = run(parsed_args) {
                eprint!("{e}");
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            e.print().expect("Printing an error should not fail.");
            match e.kind() {
                DisplayHelp | DisplayVersion => ExitCode::SUCCESS,
                _ => ExitCode::from(exit_code::INVALID_ARGS),
            }
        }
    }
}
