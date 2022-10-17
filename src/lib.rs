#![deny(
    warnings,
    nonstandard_style,
    future_incompatible,
    unused_qualifications,
    clippy::all,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::alloc_instead_of_core,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags
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
    // TODO comment out
    dead_code,
)]

extern crate alloc;
extern crate core;

use alloc::rc::Rc;
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use cursive::{
    event::{Event, EventResult, Key},
    theme::{BaseColor, BorderStyle, Color, PaletteColor},
    traits::{Nameable, Resizable},
    views::{Dialog, FixedLayout, Layer, LinearLayout, OnLayoutView, StackView, TextView},
    Cursive, Printer, Rect, Vec2, View,
};
use LocalPlayerType::Human;
use PlayerType::Local;

use crate::{
    cli::ParsedArgs,
    game::{
        ActionQueue, DefaultActionQueue, LocalPlayerType, Logic, Player, PlayerId, PlayerType,
        State, World,
    },
    tui::GameView,
    LocalPlayerType::Ai,
    ParsedArgs::{Dedicated, Interactive},
};

mod ai;
pub mod cli;
mod game;
mod lib_tests;
mod tui;
mod util;

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
    let p0 = Player::new(PlayerId::new(0), Local(Ai));
    let p1 = Player::new(PlayerId::new(1), Local(Ai));
    let p0_id = p0.id;
    let p1_id = p1.id;
    let p0_act_queue = Rc::new(DefaultActionQueue::new(p0_id));
    let p1_act_queue = Rc::new(DefaultActionQueue::new(p1_id));
    let p0_ai = ai::Random::new(
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64,
        Rc::clone(&p0_act_queue),
    );
    let p1_ai = ai::Random::new(
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64,
        Rc::clone(&p1_act_queue),
    );
    let game_world = World::new(
        State::new([p0, p1], State::DEFAULT_ROUNDS),
        Logic::new([Rc::clone(&p0_act_queue), Rc::clone(&p1_act_queue)]),
        vec![Box::new(p0_ai), Box::new(p1_ai)],
    );
    run_tui(game_world, vec![])
}

fn run_dedicated(_: ParsedArgs) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn run_tui(
    game_world: World<DefaultActionQueue>,
    action_queues: Vec<Rc<DefaultActionQueue>>,
) -> Result<(), Box<dyn Error>> {
    let mut tui = Cursive::new();
    {
        // `Color::Black` works weirdly, using `Color::RgbLowRes` instead
        let dark_black = Color::RgbLowRes(0, 0, 0);
        let light_black = Color::RgbLowRes(1, 1, 1);
        let grey = Color::RgbLowRes(3, 3, 3);
        tui.update_theme(|theme| {
            theme.shadow = true;
            theme.borders = BorderStyle::Simple;
            theme.palette[PaletteColor::Background] = Color::Light(BaseColor::White);
            theme.palette[PaletteColor::Shadow] = dark_black;
            theme.palette[PaletteColor::View] = theme.palette[PaletteColor::Background];
            theme.palette[PaletteColor::Primary] = light_black;
            theme.palette[PaletteColor::Secondary] = grey;
            theme.palette[PaletteColor::Tertiary] = Color::Dark(BaseColor::Yellow);
            theme.palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Blue);
            theme.palette[PaletteColor::TitleSecondary] = Color::Light(BaseColor::Yellow);
            theme.palette[PaletteColor::Highlight] = Color::Light(BaseColor::Green);
            theme.palette[PaletteColor::HighlightInactive] = theme.palette[PaletteColor::Highlight];
            theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Blue);
        });
    }
    let status_bar = OnLayoutView::new(
        FixedLayout::new().child(
            Rect::from_point(Vec2::zero()),
            Layer::new(TextView::new("(Esc) - exit")).full_width(),
        ),
        |layout, size| {
            let top_left_y = {
                if size.y >= 1 {
                    size.y - 1
                } else {
                    size.y
                }
            };
            layout.set_child_position(0, Rect::from_size((0, top_left_y), (size.x, 1)));
            layout.layout(size);
        },
    );
    tui.screen_mut().add_layer(
        LinearLayout::vertical()
            .child(StackView::new().layer(GameView::new(game_world, action_queues)))
            .child(status_bar),
    );
    tui.set_fps(30);
    configure_exit(&mut tui);
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
            tui.screen_mut().add_layer(
                Dialog::text("Are you sure?")
                    .title("Exit")
                    .button("No", |tui| drop(tui.pop_layer()))
                    .button("Yes", Cursive::quit)
                    .with_name(exit_dialog_id),
            );
        };
    });
}
