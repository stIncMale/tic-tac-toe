use alloc::rc::Rc;
use core::time::Duration;

use oorandom::Rand32;

use crate::{
    game::{
        Action,
        Action::Ready,
        Ai, Cell,
        Phase::{Beginning, Inround, Outround},
    },
    util::Timer,
    ActionQueue, DefaultActionQueue, PlayerId, State,
};

mod ai_tests;

#[derive(Debug)]
pub struct Random {
    rng: Rand32,
    action_queue: Rc<DefaultActionQueue>,
    base_act_delay: Option<Duration>,
    act_timer: Option<Timer>,
}

impl Random {
    pub const DEFAULT_BASE_DELAY: Duration = Duration::from_millis(700);

    pub fn new(seed: u64, action_queue: Rc<DefaultActionQueue>) -> Self {
        Self {
            rng: Rand32::new(seed),
            action_queue,
            base_act_delay: Some(Random::DEFAULT_BASE_DELAY),
            act_timer: Some(Timer::new()),
        }
    }

    pub fn set_base_act_delay(&mut self, delay: Option<Duration>) {
        if let Some(delay) = delay {
            self.base_act_delay = Some(delay);
            if let Some(act_timer) = &mut self.act_timer {
                act_timer.set_duration(delay);
            }
        } else {
            self.base_act_delay = None;
            self.act_timer = None;
        }
    }

    fn act_beginning_outround(&mut self, state: &State) {
        // By handling `act_timer` here as the first thing, we are making sure that
        // when AI plays vs. AI, there is no double waiting before becoming ready.
        if self.act_timer.as_mut().map_or(true, |act_timer| {
            act_timer.check_expired_then_unset_if_true_or_set_if_unset(state.clock.now(), || {
                Random::delay(
                    self.base_act_delay.expect("Should be `Some`."),
                    &mut self.rng,
                )
            })
        }) && state
            .required_ready
            .contains(&self.action_queue.player_id())
        {
            self.action_queue.add(Ready);
        }
    }

    fn act_inround(&mut self, state: &State) {
        if state.turn() != self.action_queue.player_id() {
            return;
        }
        if self.act_timer.as_mut().map_or(true, |act_timer| {
            act_timer.check_expired_then_unset_if_true_or_set_if_unset(state.clock.now(), || {
                Random::delay(
                    self.base_act_delay.expect("Should be `Some`."),
                    &mut self.rng,
                )
            })
        }) {
            self.action_queue
                .add(Action::Occupy(Random::decide_cell(&mut self.rng, state)));
        }
    }

    fn decide_cell(rng: &mut Rand32, state: &State) -> Cell {
        let board_size = state.board.size();
        let empty_cells_cnt = u32::try_from(board_size.pow(2)).expect("Should fit.") - state.step;
        let mut shift = rng.rand_range(0..empty_cells_cnt);
        for x in 0..board_size {
            for y in 0..board_size {
                let cell = Cell::new(x, y);
                if state.board.get(&cell).is_none() {
                    if shift == 0 {
                        return cell;
                    }
                    shift -= 1;
                }
            }
        }
        unreachable!("This method must called only if a decision is possible.")
    }

    fn delay(base: Duration, rng: &mut Rand32) -> Duration {
        base.mul_f32(1f32 + (rng.rand_float() - 0.5) / 2f32)
    }
}

impl Ai for Random {
    fn player_id(&self) -> PlayerId {
        self.action_queue.player_id()
    }

    fn act(&mut self, state: &State) {
        match state.phase {
            Beginning | Outround => self.act_beginning_outround(state),
            Inround => self.act_inround(state),
        };
    }
}
