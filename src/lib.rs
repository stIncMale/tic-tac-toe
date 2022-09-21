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
use crate::game::{ActionQueue, DefaultActionQueue, Logic, Player, PlayerId, State, World};
use crate::tui::GameView;
use crate::ParsedArgs::{Dedicated, Interactive};
use cursive::event::{Event, EventResult, Key};
use cursive::theme::Color;
use cursive::theme::{BaseColor, BorderStyle, PaletteColor};
use cursive::traits::Nameable;
use cursive::views::{Dialog, LinearLayout};
use cursive::{Cursive, Printer};
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
/// When the application must be terminated.
pub fn run(args: ParsedArgs) -> Result<(), Box<dyn Error>> {
    match args {
        Dedicated { .. } => run_dedicated(args),
        Interactive => run_interactive(args),
    }
}

fn run_interactive(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    let p0 = Player::new(0.into());
    let p1 = Player::new(1.into());
    let p0_id = p0.id;
    let p1_id = p1.id;
    let act_queue_p0 = Rc::new(DefaultActionQueue::new(p0_id));
    let act_queue_p1 = Rc::new(DefaultActionQueue::new(p1_id));
    let game_world = World::new(
        State::new([p0, p1], State::DEFAULT_ROUNDS),
        Logic::new([Rc::clone(&act_queue_p0), Rc::clone(&act_queue_p1)]),
        vec![Box::new(ai::Random::new(
            p1_id,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64,
            act_queue_p1,
        ))],
    );
    run_tui(game_world, Some(act_queue_p0))
}

fn run_dedicated(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn run_tui(
    game_world: World<DefaultActionQueue>,
    action_queue: Option<Rc<DefaultActionQueue>>,
) -> Result<(), Box<dyn Error>> {
    let mut tui = Cursive::new();
    tui.add_layer(GameView::new(game_world, action_queue));
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
