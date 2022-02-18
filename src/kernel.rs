use crate::kernel::game::{Action, ActionQueue};
use crate::PlayerId;
use std::cell::RefCell;
use std::collections::VecDeque;

pub mod game;
mod tests;

#[derive(Debug)]
pub struct DefaultActionQueue {
    player_id: PlayerId,
    actions: RefCell<VecDeque<Action>>,
}

impl DefaultActionQueue {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            actions: RefCell::new(VecDeque::with_capacity(1)),
        }
    }

    pub fn add(&self, action: Action) {
        self.actions.borrow_mut().push_back(action);
    }
}

impl ActionQueue for DefaultActionQueue {
    fn player_id(&self) -> PlayerId {
        self.player_id
    }

    fn pop(&self) -> Option<Action> {
        self.actions.borrow_mut().pop_front()
    }
}
