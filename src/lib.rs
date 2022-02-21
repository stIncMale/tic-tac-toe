#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    rust_2021_compatibility,
    clippy::all,
    clippy::pedantic
)]
#![allow(
    // unused_imports,
    dead_code,
    clippy::missing_errors_doc,
    clippy::similar_names,
    clippy::cast_possible_truncation
)]

use crate::cli::ParsedArgs;
use crate::game::Action::Ready;
use crate::game::{ActionQueue, DefaultActionQueue, Logic, Mark, Player, PlayerId, State, World};
use crate::ParsedArgs::{Dedicated, Interactive};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

mod ai;
pub mod cli;
mod game;
mod lib_tests;

pub fn run(args: ParsedArgs) -> Result<(), Box<dyn Error>> {
    match args {
        Dedicated { .. } => run_dedicated(args),
        Interactive => run_interactive(args),
    }
}

fn run_interactive(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    let px = Player::new(PlayerId::new(0), Mark::X);
    let po = Player::new(PlayerId::new(1), Mark::O);
    let px_id = px.id;
    let po_id = po.id;
    let act_queue_px = Rc::new(DefaultActionQueue::new(px_id));
    let act_queue_po = Rc::new(DefaultActionQueue::new(po_id));
    let game_logic = Logic::new([
        Rc::clone(&act_queue_px) as Rc<dyn ActionQueue>,
        Rc::clone(&act_queue_po) as Rc<dyn ActionQueue>,
    ]);
    let game_state = Rc::new(RefCell::new(State::new([px, po], 5)));
    let game_world = World::new(
        Rc::clone(&game_state),
        game_logic,
        vec![Box::new(ai::Random::new(
            po_id,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64,
            act_queue_po,
        ))],
    );
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
