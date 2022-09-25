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

use clap::ErrorKind::{DisplayHelp, DisplayVersion};
use std::env;
use std::process::ExitCode;
use tic_tac_toe_lib::cli::ParsedArgs;
use tic_tac_toe_lib::run;

/// Exit codes complementing the canonical ones in [`ExitCode`](std::process::ExitCode).
mod exit_code {
    pub const INVALID_ARGS: u8 = 2;
}

/// Panic messages may not be observable because of them printed to the terminal's
/// "alternate screen", see [here](https://github.com/gyscos/cursive/issues/409) for more details.
/// Redirect stdout to a file in order to see them.
fn main() -> ExitCode {
    match ParsedArgs::from_iterator(env::args_os()) {
        Ok(parsed_args) => {
            if let Err(e) = run(parsed_args) {
                eprint!("{}", e);
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            e.print().expect("printing an error should not fail");
            match e.kind() {
                DisplayHelp | DisplayVersion => ExitCode::SUCCESS,
                _ => ExitCode::from(exit_code::INVALID_ARGS),
            }
        }
    }
}
