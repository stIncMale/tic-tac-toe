use alloc::rc::Rc;
use core::fmt::Display;

use cursive::{
    utils::markup::StyledString,
    view::{IntoBoxedView, Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, TextView},
    Cursive,
};

use crate::{game::State, tui::util::MenuItemsStateSwitcher};

pub mod item {
    pub mod ai_vs_ai;

    pub const EXIT_LABEL: &str = "Exit";
    pub const STOP_LABEL: &str = "Stop/disconnect";
}

const ROUNDS_GAME_OPTION_VIEW_ID: &str = "ROUNDS_GAME_OPTION_VIEW_ID";
const ROUNDS_GAME_OPTION_NAME: &str = "Number of rounds";

pub fn callback<O>(
    (game_opts_dlg_title, game_opts_dlg_content): (impl Into<StyledString>, impl IntoBoxedView),
    (game_opts, start): (
        impl 'static + Fn(&mut Cursive) -> Option<O>,
        impl 'static + Fn(O, &mut Cursive),
    ),
    tui: &mut Cursive,
) {
    let menu_switcher = Rc::new(MenuItemsStateSwitcher::with_all_disabled(tui.menubar()));
    tui.screen_mut().add_layer(
        Dialog::new()
            .title(game_opts_dlg_title)
            .content(game_opts_dlg_content)
            .button("Start", {
                let menu_switcher = Rc::clone(&menu_switcher);
                move |tui| {
                    if let Some(game_opts) = game_opts(tui) {
                        drop(tui.pop_layer());
                        menu_switcher.restore(tui.menubar());
                        start(game_opts, tui);
                    }
                }
            })
            .button("Cancel", {
                let menu_switcher = Rc::clone(&menu_switcher);
                move |tui| {
                    drop(tui.pop_layer());
                    menu_switcher.restore(tui.menubar());
                }
            }),
    );
}

fn show_game_option_err_dlg(tui: &mut Cursive, opt_name: impl Display, err: impl Display) {
    tui.screen_mut().add_layer(
        Dialog::new()
            .title("Info")
            .content(TextView::new(format!("{opt_name}: {err}.")))
            .button("OK", |tui| {
                tui.pop_layer();
            }),
    );
}

fn rounds_game_option_layout() -> LinearLayout {
    let max_content_width = 15;
    LinearLayout::horizontal()
        .child(TextView::new(format!("{ROUNDS_GAME_OPTION_NAME}: ")))
        .child(
            EditView::new()
                .max_content_width(max_content_width)
                .content(State::DEFAULT_ROUNDS.to_string())
                .with_name(ROUNDS_GAME_OPTION_VIEW_ID)
                .min_width(max_content_width + 1),
        )
}
