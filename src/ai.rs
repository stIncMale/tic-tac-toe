use crate::kernel::game::Action::Ready;
use crate::kernel::game::Phase::{Beginning, Inround, Outround};
use crate::kernel::game::{Action, ActionQueue, Ai, Cell};
use crate::{PlayerId, State};
use oorandom::Rand32;
use std::cell;
use std::cell::RefCell;

mod tests;

#[derive(Debug)]
pub struct Random {
    player_id: PlayerId,
    rng: RefCell<Rand32>,
    action: cell::Cell<Option<Action>>,
}

impl Random {
    pub fn new(player_id: PlayerId, seed: u64) -> Self {
        Self {
            player_id,
            rng: RefCell::new(Rand32::new(seed)),
            action: cell::Cell::new(None),
        }
    }

    fn act_beginning_outround(&self, state: &State) {
        if state.required_ready.contains(&self.player_id) {
            self.action.set(Some(Ready));
        };
    }

    fn act_inround(&self, state: &State) {
        if state.turn() != self.player_id {
            return;
        }
        let board_size = state.board.size();
        let empty_cells_cnt = u32::try_from(board_size.pow(2)).unwrap() - state.step;
        let mut shift = self.rng.borrow_mut().rand_range(0..empty_cells_cnt);
        for x in 0..board_size {
            for y in 0..board_size {
                let cell = Cell::new(x, y);
                if state.board.get(&cell).is_none() {
                    if shift == 0 {
                        self.action.set(Some(Action::Occupy(cell)));
                        return;
                    }
                    shift -= 1;
                }
            }
        }
    }
}

impl Ai for Random {
    fn act(&self, state: &State) {
        if self.action.get().is_some() {
            return;
        }
        match state.phase {
            Beginning | Outround => self.act_beginning_outround(state),
            Inround => self.act_inround(state),
        };
    }
}

impl ActionQueue for Random {
    fn player_id(&self) -> PlayerId {
        self.player_id
    }

    fn pop(&self) -> Option<Action> {
        self.action.take()
    }
}
