#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    rust_2021_compatibility,
    clippy::all,
    clippy::pedantic
)]
#![allow(dead_code, clippy::missing_errors_doc, clippy::similar_names)]

use crate::cli::ParsedArgs;
use crate::kernel::game::{Logic, Mark, Player, PlayerId, State, World};
use crate::kernel::DefaultActionQueue;
use crate::ParsedArgs::{Dedicated, Interactive};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

mod ai;
pub mod cli;
mod kernel;
mod tests;

pub fn run(args: ParsedArgs) -> Result<(), Box<dyn Error>> {
    match args {
        Dedicated { .. } => run_dedicated(args),
        Interactive => {
            run_interactive(args);
            Ok(())
        }
    }
}

fn run_interactive(_: ParsedArgs) {
    let px = Player::new(PlayerId::new(0), Mark::X);
    let po = Player::new(PlayerId::new(1), Mark::O);
    let game_state = Rc::new(RefCell::new(State::new([px, po], 5)));
    let act_queue_px = DefaultActionQueue::new();
    let act_queue_po = ai::Random::new(Rc::clone(&game_state));
    let game_logic = Logic::new([&act_queue_px, &act_queue_po]);
    let game_world = World::new(Rc::clone(&game_state), game_logic);
    for _ in 0..10 {
        game_world.advance();
    }
}

fn run_dedicated(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    todo!()
}
