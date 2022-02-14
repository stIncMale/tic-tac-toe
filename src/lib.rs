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
use crate::kernel::game::Action::{Occupy, Ready};
use crate::kernel::game::{Cell, Logic, Mark, Player, PlayerId, State, World};
use crate::kernel::DefaultActionQueue;
use crate::ParsedArgs::{Dedicated, Interactive};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

mod ai;
pub mod cli;
mod kernel;
mod tests;

pub fn run(args: ParsedArgs) -> Result<(), Box<dyn Error>> {
    match args {
        Dedicated { .. } => run_dedicated(args),
        Interactive => run_interactive(args),
    }
}

fn run_interactive(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    let px = Player::new(PlayerId::new(0), Mark::X);
    let po = Player::new(PlayerId::new(1), Mark::O);
    let po_id = po.id;
    let game_state = Rc::new(RefCell::new(State::new([px, po], 5)));
    let act_queue_px = DefaultActionQueue::new();
    let act_queue_po = ai::Random::new(
        po_id,
        Rc::clone(&game_state),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
    );
    let game_logic = Logic::new([&act_queue_px, &act_queue_po]);
    let game_world = World::new(Rc::clone(&game_state), game_logic);
    {
        act_queue_px.add(Ready);
        act_queue_po.act();
        game_world.advance();
        game_world.advance();
        println!("{:?}", game_state.borrow());

        act_queue_px.add(Occupy(Cell::new(0, 0)));
        act_queue_po.act();
        game_world.advance();
        game_world.advance();
        println!("{:?}", game_state.borrow());

        act_queue_px.add(Occupy(Cell::new(1, 1)));
        act_queue_po.act();
        game_world.advance();
        game_world.advance();
        println!("{:?}", game_state.borrow());

        act_queue_px.add(Occupy(Cell::new(2, 2)));
        act_queue_po.act();
        game_world.advance();
        game_world.advance();
        println!("{:?}", game_state.borrow());
    }
    Ok(())
}

fn run_dedicated(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    todo!()
}
