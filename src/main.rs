#![deny(warnings, clippy::all, clippy::pedantic)]
#![allow(dead_code)]

mod libs;

use crate::libs::{run, Config};
use std::{env, process};

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
