use alloc::rc::Rc;
use core::{fmt::Display, num::ParseIntError};
use std::time::{SystemTime, UNIX_EPOCH};

use cursive::{
    views::{EditView, LinearLayout, NamedView},
    Cursive,
};

use crate::{
    ai::RandomAi,
    game::{
        DefaultActionQueue, LocalPlayerType::Ai, Logic, Player, PlayerId, PlayerType::Local, State,
        World,
    },
    tui::{
        menu::{
            rounds_game_option_layout, show_game_option_err_dlg, ROUNDS_GAME_OPTION_NAME,
            ROUNDS_GAME_OPTION_VIEW_ID,
        },
        view::GameView,
    },
};

#[derive(Debug)]
pub struct GameOpts {
    _rounds: u32,
}

impl GameOpts {
    fn from_parsed(
        (rounds_option_name, rounds): (impl Display, Result<u32, ParseIntError>),
        tui: &mut Cursive,
    ) -> Option<Self> {
        if let Ok(rounds) = rounds {
            Some(Self { _rounds: rounds })
        } else if let Err(err) = rounds {
            show_game_option_err_dlg(tui, rounds_option_name, err);
            None
        } else {
            None
        }
    }
}

pub fn game_opts_dlg_content() -> LinearLayout {
    rounds_game_option_layout()
}

pub fn game_opts(tui: &mut Cursive) -> Option<GameOpts> {
    GameOpts::from_parsed(
        (
            ROUNDS_GAME_OPTION_NAME,
            tui.call_on_name(
                ROUNDS_GAME_OPTION_VIEW_ID,
                |view: &mut NamedView<EditView>| view.get_mut().get_content().parse::<u32>(),
            )
            .unwrap(),
        ),
        tui,
    )
}

pub fn start(_: GameOpts, tui: &mut Cursive) {
    let p0 = Player::new(PlayerId::new(0), Local(Ai));
    let p1 = Player::new(PlayerId::new(1), Local(Ai));
    let p0_id = p0.id;
    let p1_id = p1.id;
    let p0_act_queue = Rc::new(DefaultActionQueue::new(p0_id));
    let p1_act_queue = Rc::new(DefaultActionQueue::new(p1_id));
    let p0_ai = RandomAi::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
        Rc::clone(&p0_act_queue),
    );
    let p1_ai = RandomAi::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
        Rc::clone(&p1_act_queue),
    );
    let game_world = World::new(
        State::new([p0, p1], State::DEFAULT_ROUNDS),
        Logic::new([Rc::clone(&p0_act_queue), Rc::clone(&p1_act_queue)]),
        vec![Box::new(p0_ai), Box::new(p1_ai)],
    );
    tui.screen_mut()
        .add_fullscreen_layer(GameView::new(game_world, vec![]));
}
