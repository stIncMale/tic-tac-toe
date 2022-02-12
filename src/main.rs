#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    rust_2021_compatibility,
    clippy::all,
    clippy::pedantic
)]
#![allow(dead_code, clippy::missing_errors_doc, clippy::similar_names)]

use std::{env, process};
use tic_tac_toe::{run, ParsedArgs};

fn main() {
    let args = env::args()
        // skip the path of the executable
        .skip(1);
    let parsed_args = ParsedArgs::new(args).unwrap_or_else(|e| {
        eprintln!("Invalid arguments. {}", e);
        process::exit(1);
    });
    if let Err(e) = run(parsed_args) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
