#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    unused_qualifications,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::alloc_instead_of_core,
    clippy::use_self,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags
)]
#![allow(
    clippy::similar_names,
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
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
use tic_tac_toe_lib::{
    cli::ParsedArgs,
    process::{setup_ctrlc_handler, setup_panic, MoreExitCode},
    run,
};

fn main() -> ExitCode {
    setup_panic();
    let exit_signal = setup_ctrlc_handler();
    match ParsedArgs::try_from_iterator(env::args_os()) {
        Ok(parsed_args) => {
            if let Err(e) = run(parsed_args, &exit_signal) {
                eprint!("{e}");
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            e.print().expect("printing an error should not fail");
            match e.kind() {
                DisplayHelp | DisplayVersion => ExitCode::SUCCESS,
                _ => ExitCode::from(MoreExitCode::INVALID_ARGS),
            }
        }
    }
}
