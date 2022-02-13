use crate::kernel::game::{Action, ActionQueue};
use std::cell::RefCell;
use std::collections::VecDeque;

pub mod game;
mod tests;

#[derive(Debug)]
pub struct DefaultActionQueue {
    actions: RefCell<VecDeque<Action>>,
}

impl DefaultActionQueue {
    pub fn new() -> Self {
        Self {
            actions: RefCell::new(VecDeque::with_capacity(1)),
        }
    }

    pub fn add(&self, action: Action) {
        self.actions.borrow_mut().push_back(action);
    }
}

impl ActionQueue for DefaultActionQueue {
    fn pop(&self) -> Option<Action> {
        self.actions.borrow_mut().pop_front()
    }
}
