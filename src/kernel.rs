use crate::kernel::game as g;
use std::cell::RefCell;
use std::collections::VecDeque;

pub mod game;
mod tests;

#[derive(Debug)]
pub struct DefaultActionQueue {
    actions: RefCell<VecDeque<g::Action>>,
}

impl DefaultActionQueue {
    pub fn new() -> Self {
        Self {
            actions: RefCell::new(VecDeque::with_capacity(1)),
        }
    }

    pub fn add(&self, action: g::Action) {
        self.actions.borrow_mut().push_back(action);
    }
}

impl g::ActionQueue for DefaultActionQueue {
    fn pop(&self) -> Option<g::Action> {
        self.actions.borrow_mut().pop_front()
    }
}
