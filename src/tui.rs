use std::error::Error;

use cursive::{
    event::{Event, Key},
    menu::Tree,
    theme::{BaseColor, BorderStyle, Color, PaletteColor},
    views::{Dialog, Panel},
    Cursive,
};

use crate::{
    tui::{
        util::{MenuItemSwitchState::Disabled, MenuItemsStateSwitcher},
        view::SplashScreenView,
    },
    APP_METADATA,
};

mod fx;
mod menu;
mod util;
mod view;

// TODO use https://crates.io/crates/anyhow?
pub fn run() -> Result<(), Box<dyn Error>> {
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
                        .leaf("Couch", |_| todo!())
                        .leaf("Connect", |_| todo!())
                        .leaf("Host", |_| todo!()),
                )
                .leaf(menu::STOP_LABEL, |_| todo!())
                .leaf(menu::EXIT_LABEL, Cursive::quit),
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

fn show_about_dlg(tui: &mut Cursive) {
    let menu_switcher = MenuItemsStateSwitcher::with_all_disabled(tui.menubar());
    tui.screen_mut().add_layer(
        Dialog::text(format!(
            "{app} {version}.\n\
        \n\
        For more info run `{exe} --help`.",
            app = APP_METADATA.name,
            version = APP_METADATA.version,
            exe = APP_METADATA.exe
        ))
        .title("About")
        .button("Close", move |tui| {
            drop(tui.pop_layer());
            menu_switcher.restore(tui.menubar());
        }),
    );
}
