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
    util::time::Timer,
    ActionQueue, DefaultActionQueue, PlayerId, State,
};

mod test;

pub const DEFAULT_BASE_DELAY: Duration = Duration::from_millis(700);

#[derive(Debug)]
pub struct RandomAi {
    rng: Rand32,
    action_queue: Rc<DefaultActionQueue>,
    base_act_delay: Duration,
    act_timer: Timer,
}

impl RandomAi {
    pub fn new(seed: u64, action_queue: Rc<DefaultActionQueue>) -> Self {
        Self {
            rng: Rand32::new(seed),
            action_queue,
            base_act_delay: DEFAULT_BASE_DELAY,
            act_timer: Timer::new(),
        }
    }

    fn act_beginning_outround(&mut self, state: &State) {
        // By handling `act_timer` here as the first thing, we are making sure that
        // when AI plays vs. AI, there is no double waiting before becoming ready.
        if self.can_act(state)
            && state
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
        if self.can_act(state) {
            self.action_queue
                .add(Action::Occupy(Self::decide_cell(&mut self.rng, state)));
        }
    }

    fn decide_cell(rng: &mut Rand32, state: &State) -> Cell {
        let board_size = state.board.size();
        let empty_cells_cnt = u32::try_from(board_size.pow(2)).unwrap() - state.step;
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

    fn can_act(&mut self, state: &State) -> bool {
        self.base_act_delay.is_zero()
            || self
                .act_timer
                .check_expired_then_unset_if_true_or_set_if_unset(
                    state.clock.as_ref().unwrap().now(),
                    || Self::delay(self.base_act_delay, &mut self.rng),
                )
    }

    fn delay(base: Duration, rng: &mut Rand32) -> Duration {
        base.mul_f32(1.0 + (rng.rand_float() - 0.5) / 2.0)
    }
}

impl Ai for RandomAi {
    fn player_id(&self) -> PlayerId {
        self.action_queue.player_id()
    }

    fn act(&mut self, state: &State) {
        match state.phase {
            Beginning | Outround => self.act_beginning_outround(state),
            Inround => self.act_inround(state),
        };
    }

    fn set_base_act_delay(&mut self, delay: Duration) {
        self.base_act_delay = delay;
        self.act_timer
            .set_duration(Self::delay(self.base_act_delay, &mut self.rng));
    }
}
