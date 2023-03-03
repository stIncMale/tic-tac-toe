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
// TODO remove when https://github.com/rust-lang/rust/issues/103765 is done,
// also remove +nightly from cargo test, build, run commands.
#![feature(error_in_core)]
// TODO remove when https://github.com/rust-lang/rust/issues/102929 is done,
// also remove +nightly from cargo test, build, run commands.
#![feature(string_leak)]

extern crate alloc;
extern crate core;

use alloc::sync::Arc;
use core::error::Error;

use cursive::{
    event::{Event, EventResult},
    views::{LinearLayout, TextView},
    Cursive, Printer,
};
use LocalPlayerType::Human;
use PlayerType::Local;

use crate::{
    cli::ParsedArgs,
    game::{
        ActionQueue, DefaultActionQueue, LocalPlayerType, Logic, Player, PlayerId, PlayerType,
        State, World,
    },
    process::ExitSignal,
    ParsedArgs::{Dedicated, Interactive},
};

mod ai;
pub mod cli;
mod game;
pub mod process;
mod server;
mod test;
mod tui;
mod util;

/// # Errors
///
/// When the application must be terminated.
pub fn run(args: ParsedArgs, exit_signal: &Arc<ExitSignal>) -> Result<(), Box<dyn Error>> {
    match args {
        Dedicated { .. } => server::run(args, exit_signal),
        Interactive => tui::run(exit_signal),
    }
}
