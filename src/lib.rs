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

extern crate alloc;
extern crate core;

use core::error::Error;
use std::{env, ffi::OsStr};

use cursive::{
    event::{Event, EventResult},
    views::{LinearLayout, TextView},
    Cursive, Printer,
};
use once_cell::sync::Lazy;
use LocalPlayerType::Human;
use PlayerType::Local;

use crate::{
    cli::ParsedArgs,
    game::{
        ActionQueue, DefaultActionQueue, LocalPlayerType, Logic, Player, PlayerId, PlayerType,
        State, World,
    },
    ParsedArgs::{Dedicated, Interactive},
};

mod ai;
pub mod cli;
mod game;
mod test;
mod tui;
mod util;

// TODO refactor when https://github.com/rust-lang/rust/issues/74465 is done,
// also remove the once_cell dependency.
pub static APP_METADATA: Lazy<AppMetadata> = Lazy::new(|| AppMetadata {
    name: "Tic-tac-toe",
    version: env!("CARGO_PKG_VERSION"),
    authors: env!("CARGO_PKG_AUTHORS"),
    homepage: env!("CARGO_PKG_REPOSITORY"),
    exe: {
        let fallback = "<game-executable>";
        env::current_exe()
            .as_ref()
            .map(|path| path.file_name())
            .map(|name| name.and_then(OsStr::to_str).unwrap_or(fallback))
            .unwrap_or(fallback)
            .to_owned()
    },
});

pub struct AppMetadata {
    pub name: &'static str,
    pub version: &'static str,
    pub authors: &'static str,
    pub homepage: &'static str,
    pub exe: String,
}

/// # Errors
///
/// When the application must be terminated.
pub fn run(args: ParsedArgs) -> Result<(), Box<dyn Error>> {
    match args {
        Dedicated { .. } => run_dedicated(args),
        Interactive => tui::run(),
    }
}

fn run_dedicated(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    todo!()
}
