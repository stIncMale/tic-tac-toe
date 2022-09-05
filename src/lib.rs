#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    unused_qualifications,
    clippy::all,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    // TODO uncomment in Clippy 1.64
    // clippy::std_instead_of_core,
    // clippy::std_instead_of_alloc,
    // clippy::alloc_instead_of_core,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
)]
#![allow(
    clippy::similar_names,
    clippy::cast_possible_truncation,
    // uncomment below to simplify editing, comment out again before committing
    // clippy::pedantic,
    // unused_imports,
    // unused_variables,
    // unused_mut,
    // unreachable_code,
    // dead_code,
)]

use crate::cli::ParsedArgs;
use crate::game::{ActionQueue, DefaultActionQueue, Logic, Mark, Player, PlayerId, State, World};
use crate::tui::GameView;
use crate::ParsedArgs::{Dedicated, Interactive};
use cursive::event::{Event, EventResult, Key};
use cursive::theme::Color;
use cursive::theme::{BaseColor, BorderStyle, PaletteColor};
use cursive::traits::Nameable;
use cursive::views::{Dialog, LinearLayout};
use cursive::{Cursive, Printer};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

mod ai;
pub mod cli;
mod game;
mod lib_tests;
mod tui;

/// # Errors
///
/// If [`Result::Err`] is returned, then the whole application must be terminated.
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
    let game_state = Rc::new(RefCell::new(State::new([px, po], State::DEFAULT_ROUNDS)));
    let game_world = World::new(
        Rc::clone(&game_state),
        Logic::new([
            Rc::clone(&act_queue_px) as Rc<dyn ActionQueue>,
            Rc::clone(&act_queue_po) as Rc<dyn ActionQueue>,
        ]),
        vec![Box::new(ai::Random::new(
            po_id,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64,
            act_queue_po,
        ))],
    );
    run_tui(&game_state, Some(act_queue_px), game_world)
}

fn run_dedicated(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn run_tui(
    game_state: &Rc<RefCell<State>>,
    action_queue: Option<Rc<DefaultActionQueue>>,
    game_world: World,
) -> Result<(), Box<dyn Error>> {
    let mut tui = Cursive::new();
    tui.add_layer(GameView::new(
        game_state,
        action_queue,
        Box::new(move || game_world.advance()),
    ));
    configure_exit(&mut tui);
    tui.update_theme(|theme| {
        theme.shadow = true;
        theme.borders = BorderStyle::Simple;
        theme.palette[PaletteColor::Background] = Color::Rgb(255, 255, 255);
        theme.palette[PaletteColor::View] = theme.palette[PaletteColor::Background];
        theme.palette[PaletteColor::Highlight] = Color::Light(BaseColor::Black);
        theme.palette[PaletteColor::HighlightInactive] = theme.palette[PaletteColor::Highlight];
        theme.palette[PaletteColor::Secondary] = theme.palette[PaletteColor::Highlight];
        theme.palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Blue);
    });
    tui.set_fps(20);
    tui.try_run_with::<Box<dyn Error>, _>(|| {
        // work around https://github.com/gyscos/Cursive/issues/142
        Ok(Box::new(cursive_buffered_backend::BufferedBackend::new(
            cursive::backends::crossterm::Backend::init()?,
        )))
    })
}

fn configure_exit(tui: &mut Cursive) {
    tui.add_global_callback(Event::Key(Key::Esc), |tui| {
        let exit_dialog_id = "exit_dialog_id";
        if tui.find_name::<Dialog>(exit_dialog_id).is_none() {
            tui.add_layer(
                Dialog::text("Are you sure?")
                    .title("Exit")
                    .button("No", |tui| drop(tui.pop_layer()))
                    .button("Yes", Cursive::quit)
                    .with_name(exit_dialog_id),
            );
        };
    });
}
