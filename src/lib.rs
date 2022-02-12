#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    rust_2021_compatibility,
    clippy::all,
    clippy::pedantic
)]
#![allow(dead_code, clippy::missing_errors_doc, clippy::similar_names)]

use crate::kernel as k;
use crate::kernel::game as g;
use crate::Mode::{Dedicated, Interactive};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

mod ai;
mod kernel;
mod tests;

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Interactive,
    Dedicated,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedArgs {
    mode: Mode,
}

impl ParsedArgs {
    pub fn new<T>(mut args: T) -> Result<ParsedArgs, &'static str>
    where
        T: Iterator<Item = String>,
    {
        match args.next() {
            Some(_) => Err("Arguments are not supported."),
            None => Ok(ParsedArgs { mode: Interactive }),
        }
    }
}

pub fn run(args: ParsedArgs) -> Result<(), Box<dyn Error>> {
    match args.mode {
        Interactive => {
            run_interactive(args);
            Ok(())
        }
        Dedicated => run_dedicated(args),
    }
}

fn run_interactive(_: ParsedArgs) {
    let px = g::Player::new(g::PlayerId::new(0), g::Mark::X);
    let po = g::Player::new(g::PlayerId::new(1), g::Mark::O);
    let game_state = Rc::new(RefCell::new(g::State::new([px, po], 5)));
    let act_queue_px = k::DefaultActionQueue::new();
    let act_queue_po = ai::Random::new(Rc::clone(&game_state));
    let game_logic = g::Logic::new([&act_queue_px, &act_queue_po]);
    let game_world = g::World::new(Rc::clone(&game_state), game_logic);
    for _ in 0..10 {
        game_world.advance();
    }
}

fn run_dedicated(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    todo!()
}
