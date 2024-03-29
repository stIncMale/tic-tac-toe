use alloc::sync::Arc;
use std::error::Error;

use cursive::{
    event::{Event, Key},
    menu::Tree,
    theme::{BaseColor, BorderStyle, Color, PaletteColor},
    views::{Dialog, Panel},
    Cursive,
};

use crate::{
    process::{ExitSignal, APP_METADATA},
    tui::{
        util::{MenuItemSwitchState::Disabled, MenuItemsStateSwitcher},
        view::SplashScreenView,
    },
};

mod fx;
mod menu;
mod util;
mod view;

// TODO use https://crates.io/crates/anyhow?
pub fn run(exit_signal: &Arc<ExitSignal>) -> Result<(), Box<dyn Error>> {
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
                        .leaf("TODO Vs. AI", |_| todo!())
                        .leaf("AI vs. AI", |tui| {
                            menu::callback(
                                ("AI vs. AI options", menu::ai_vs_ai::game_opts_dlg_content()),
                                (menu::ai_vs_ai::game_opts, menu::ai_vs_ai::start),
                                tui,
                            );
                        }),
                )
                .subtree(
                    "Multiplayer",
                    Tree::new()
                        .leaf("TODO Couch", |_| todo!())
                        .leaf("TODO Connect", |_| todo!())
                        .leaf("TODO Host", |_| todo!()),
                )
                .leaf(menu::STOP_LABEL, |_| todo!())
                .leaf(menu::EXIT_LABEL, exit),
        )
        .add_subtree("Help", Tree::new().leaf("About", show_about_dlg));
    MenuItemsStateSwitcher::new().switch(tui.menubar(), |lbl| {
        if lbl == menu::STOP_LABEL {
            Some(Disabled)
        } else {
            None
        }
    });
    tui.add_global_callback(Event::Key(Key::Esc), Cursive::select_menubar);
    {
        // `cursive` handles the Ctrl+C combination on its own,
        // but it is not documented, and the corresponding signal (e.g., `SIGINT` in POSIX)
        // is not handled, so we handle it.
        let exit_signal = Arc::clone(exit_signal);
        tui.add_global_callback(Event::Refresh, move |tui| {
            if exit_signal.is_received() {
                exit(tui);
            }
        });
    }
    tui.set_autohide_menu(false);
    tui.screen_mut()
        .add_fullscreen_layer(Panel::new(SplashScreenView::new()));
    tui.set_fps(30);
    tui.try_run_with::<Box<dyn Error>, _>(|| {
        // work around https://github.com/gyscos/Cursive/issues/142
        Ok(Box::new(cursive_buffered_backend::BufferedBackend::new(
            cursive::backends::crossterm::Backend::init()?,
        )))
    })
}

fn exit(tui: &mut Cursive) {
    tui.quit();
}

fn show_about_dlg(tui: &mut Cursive) {
    let menu_switcher = MenuItemsStateSwitcher::with_all_disabled(tui.menubar());
    tui.screen_mut().add_layer(
        Dialog::text(format!(
            "{app} {version}.\n\
        \n\
        For more info run `{exe} --help`.",
            app = APP_METADATA.name(),
            version = APP_METADATA.version(),
            exe = APP_METADATA.exe()
        ))
        .title("About")
        .button("Close", move |tui| {
            drop(tui.pop_layer());
            menu_switcher.restore(tui.menubar());
        }),
    );
}
