#![deny(warnings, clippy::all, clippy::pedantic)]
#![allow(dead_code, clippy::missing_errors_doc, clippy::similar_names)]

use std::{env, process};
use tic_tac_toe::{run, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
