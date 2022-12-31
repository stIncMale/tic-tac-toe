use alloc::rc::Rc;
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use cursive::{
    event::{Event, Key},
    menu::Tree,
    theme::{BaseColor, BorderStyle, Color, PaletteColor},
    views::Dialog,
    Cursive,
};

use crate::{
    ai::RandomAi,
    game::{
        DefaultActionQueue, LocalPlayerType::Ai, Logic, Player, PlayerId, PlayerType::Local, State,
        World,
    },
    tui::{util::MenuLeafItemsStateSwitcher, view::GameView},
    APP_INFO,
};

mod fx;
mod util;
mod view;

const EXIT_MENU_ITEM_LABEL: &str = "Exit";

pub fn run() -> Result<(), Box<dyn Error>> {
    let p0 = Player::new(PlayerId::new(0), Local(Ai));
    let p1 = Player::new(PlayerId::new(1), Local(Ai));
    let p0_id = p0.id;
    let p1_id = p1.id;
    let p0_act_queue = Rc::new(DefaultActionQueue::new(p0_id));
    let p1_act_queue = Rc::new(DefaultActionQueue::new(p1_id));
    let p0_ai = RandomAi::new(
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64,
        Rc::clone(&p0_act_queue),
    );
    let p1_ai = RandomAi::new(
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

fn run_tui(
    game_world: World<DefaultActionQueue>,
    action_queues: Vec<Rc<DefaultActionQueue>>,
) -> Result<(), Box<dyn Error>> {
    let mut tui = Cursive::new();
    {
        // `Color::Black` works weirdly, using `Color::RgbLowRes` instead
        let dark_black = Color::RgbLowRes(0, 0, 0);
        let grey = Color::RgbLowRes(3, 3, 3);
        tui.update_theme(|theme| {
            theme.shadow = true;
            theme.borders = BorderStyle::Simple;
            theme.palette[PaletteColor::Background] = dark_black;
            theme.palette[PaletteColor::Shadow] = dark_black;
            theme.palette[PaletteColor::View] = Color::Light(BaseColor::White);
            theme.palette[PaletteColor::Primary] = dark_black;
            theme.palette[PaletteColor::Secondary] = grey;
            theme.palette[PaletteColor::Tertiary] = Color::Dark(BaseColor::Yellow);
            theme.palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Blue);
            theme.palette[PaletteColor::TitleSecondary] = Color::Light(BaseColor::Yellow);
            theme.palette[PaletteColor::Highlight] = Color::Light(BaseColor::Green);
            theme.palette[PaletteColor::HighlightInactive] = theme.palette[PaletteColor::Highlight];
            theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Blue);
        });
    }
    tui.menubar()
        .add_subtree(
            "Game",
            Tree::new()
                .subtree(
                    "Singleplayer",
                    Tree::new()
                        .leaf("Vs. AI", |_| todo!())
                        .leaf("AI vs. AI", |_| todo!()),
                )
                .subtree(
                    "Multiplayer",
                    Tree::new()
                        .leaf("Couch", |_| todo!())
                        .leaf("Connect", |_| todo!())
                        .leaf("Host", |_| todo!()),
                )
                .leaf("Stop/disconnect", |_| todo!())
                .leaf(EXIT_MENU_ITEM_LABEL, Cursive::quit),
        )
        .add_subtree(
            "Help",
            Tree::new().leaf("About", |tui| {
                let menu_switcher = {
                    let mut menu_switcher = MenuLeafItemsStateSwitcher::new();
                    menu_switcher.disable_all(tui.menubar());
                    menu_switcher
                };
                tui.screen_mut().add_layer(
                    Dialog::text(format!(
                        "{app} {version}.\n\
                        \n\
                        For more info run `{exe} --help`.",
                        app = APP_INFO.name,
                        version = APP_INFO.version,
                        exe = APP_INFO.exe
                    ))
                    .title("About")
                    .button("Close", move |tui| {
                        drop(tui.pop_layer());
                        menu_switcher.restore(tui.menubar());
                    }),
                );
            }),
        );
    tui.add_global_callback(Event::Key(Key::Esc), Cursive::select_menubar);
    tui.set_autohide_menu(false);
    tui.screen_mut()
        .add_layer(GameView::new(game_world, action_queues));
    tui.set_fps(30);
    tui.try_run_with::<Box<dyn Error>, _>(|| {
        // work around https://github.com/gyscos/Cursive/issues/142
        Ok(Box::new(cursive_buffered_backend::BufferedBackend::new(
            cursive::backends::crossterm::Backend::init()?,
        )))
    })
}
