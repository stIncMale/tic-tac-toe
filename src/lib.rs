#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    rust_2021_compatibility,
    clippy::all,
    clippy::pedantic
)]
#![allow(
    dead_code,
    clippy::missing_errors_doc,
    clippy::similar_names,
    clippy::cast_possible_truncation
)]

use crate::cli::ParsedArgs;
use crate::kernel::game::Action::Ready;
use crate::kernel::game::{ActionQueue, Logic, Mark, Player, PlayerId, State, World};
use crate::kernel::DefaultActionQueue;
use crate::ParsedArgs::{Dedicated, Interactive};
use std::borrow::Borrow;
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
    let act_queue_px = Rc::new(DefaultActionQueue::new());
    let act_queue_po = Rc::new(ai::Random::new(
        po_id,
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64,
    ));
    let game_logic = Logic::new([
        Rc::clone(&act_queue_px) as Rc<dyn ActionQueue>,
        Rc::clone(&act_queue_po) as Rc<dyn ActionQueue>,
    ]);
    let game_world = World::new(Rc::clone(&game_state), game_logic, vec![act_queue_po]);
    {
        act_queue_px.add(Ready);
        game_world.advance();
        game_world.advance();
    }
    let x: &RefCell<State> = game_state.borrow();
    let y: &State = &*x.borrow();
    println!("{:?}", y);

    Ok(())
}

fn run_dedicated(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    todo!()
}
