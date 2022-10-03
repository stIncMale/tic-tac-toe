use alloc::rc::Rc;

use oorandom::Rand32;

use crate::{
    game::{
        Action,
        Action::Ready,
        Ai, Cell,
        Phase::{Beginning, Inround, Outround},
    },
    ActionQueue, DefaultActionQueue, PlayerId, State,
};

mod ai_tests;

#[derive(Debug)]
pub struct Random {
    rng: Rand32,
    action_queue: Rc<DefaultActionQueue>,
}

impl Random {
    pub fn new(seed: u64, action_queue: Rc<DefaultActionQueue>) -> Self {
        Self {
            rng: Rand32::new(seed),
            action_queue,
        }
    }

    fn act_beginning_outround(&mut self, state: &State) {
        if state
            .required_ready
            .contains(&self.action_queue.player_id())
        {
            self.action_queue.add(Ready);
        };
    }

    fn act_inround(&mut self, state: &State) {
        if state.turn() != self.action_queue.player_id() {
            return;
        }
        let board_size = state.board.size();
        let empty_cells_cnt = u32::try_from(board_size.pow(2)).expect("should fit") - state.step;
        let mut shift = self.rng.rand_range(0..empty_cells_cnt);
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
