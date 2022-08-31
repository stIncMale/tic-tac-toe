#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    rust_2021_compatibility,
    unused_qualifications,
    clippy::all,
    clippy::pedantic
)]
#![allow(
    clippy::missing_errors_doc,
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

use std::env;
use std::process::ExitCode;
use tic_tac_toe::cli::ParsedArgs;
use tic_tac_toe::run;

/// Exit codes complementing the canonical ones in [`ExitCode`](std::process::ExitCode).
mod exit_code {
    pub const INVALID_ARGS: u8 = 3;
}

/// Panic messages may not be observable because of them being printed to the terminal's
/// "alternate screen", see [here](https://github.com/gyscos/cursive/issues/409) for more details.
/// Redirect stdout to a file in order to see them.
fn main() -> ExitCode {
    match ParsedArgs::from_iterator(env::args_os()) {
        Ok(parsed_args) => {
            if let Err(e) = run(parsed_args) {
                eprintln!("{}", e);
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::from(exit_code::INVALID_ARGS)
        }
    }
}
