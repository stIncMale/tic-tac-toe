use crate::kernel::game::Action::Ready;
use crate::kernel::game::Phase::{Beginning, Inround, Outround};
use crate::kernel::game::{Action, ActionQueue, Cell};
use crate::{PlayerId, State};
use oorandom::Rand32;
use std::cell;
use std::cell::RefCell;
use std::rc::Rc;

mod tests;

#[derive(Debug)]
pub struct Random {
    player_id: PlayerId,
    state: Rc<RefCell<State>>,
    rng: RefCell<Rand32>,
    action: cell::Cell<Option<Action>>,
}

impl Random {
    pub fn new(player_id: PlayerId, state: Rc<RefCell<State>>, seed: u64) -> Self {
        Self {
            player_id,
            state,
            rng: RefCell::new(Rand32::new(seed)),
            action: cell::Cell::new(None),
        }
    }

    pub fn act(&self) {
        if self.action.get().is_some() {
            return;
        }
        let state = &*self.state.borrow();
        match state.phase {
            Beginning | Outround => self.act_beginning_outround(state),
            Inround => self.act_inround(state),
        };
    }

    fn act_beginning_outround(&self, state: &State) {
        if state.required_ready.contains(&self.player_id) {
            self.action.set(Some(Ready));
        };
    }

    fn act_inround(&self, state: &State) {
        let board = &state.board;
        let board_size = board.size();
        let mut empty_cells = Vec::with_capacity(board_size * board_size);
        for x in 0..board_size {
            for y in 0..board_size {
                let cell = Cell::new(x, y);
                if board.get(&cell).is_none() {
                    empty_cells.push(cell);
                }
            }
        }
        assert!(!empty_cells.is_empty());
        let rand_empty_cell = *empty_cells
            .get(
                usize::try_from(
                    self.rng
                        .borrow_mut()
                        .rand_range(0..u32::try_from(empty_cells.len()).unwrap()),
                )
                .unwrap(),
            )
            .unwrap();
        self.action.set(Some(Action::Occupy(rand_empty_cell)));
    }
}

impl ActionQueue for Random {
    fn pop(&self) -> Option<Action> {
        self.action.take()
    }
}
