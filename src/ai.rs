use crate::game::Action::Ready;
use crate::game::Phase::{Beginning, Inround, Outround};
use crate::game::{Action, Ai, Cell};
use crate::{DefaultActionQueue, PlayerId, State};
use oorandom::Rand32;
use std::cell::RefCell;
use std::rc::Rc;

mod ai_tests;

#[derive(Debug)]
pub struct Random {
    player_id: PlayerId,
    rng: RefCell<Rand32>,
    action_queue: Rc<DefaultActionQueue>,
}

impl Random {
    pub fn new(player_id: PlayerId, seed: u64, action_queue: Rc<DefaultActionQueue>) -> Self {
        Self {
            player_id,
            rng: RefCell::new(Rand32::new(seed)),
            action_queue,
        }
    }

    fn act_beginning_outround(&self, state: &State) {
        if state.required_ready.contains(&self.player_id) {
            self.action_queue.add(Ready);
        };
    }

    fn act_inround(&self, state: &State) {
        if state.turn() != self.player_id {
            return;
        }
        let board_size = state.board.size();
        let empty_cells_cnt = u32::try_from(board_size.pow(2)).expect("should fit") - state.step;
        let mut shift = self.rng.borrow_mut().rand_range(0..empty_cells_cnt);
        for x in 0..board_size {
            for y in 0..board_size {
                let cell = Cell::new(x, y);
                if state.board.get(&cell).is_none() {
                    if shift == 0 {
                        self.action_queue.add(Action::Occupy(cell));
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
        match state.phase {
            Beginning | Outround => self.act_beginning_outround(state),
            Inround => self.act_inround(state),
        };
    }
}
