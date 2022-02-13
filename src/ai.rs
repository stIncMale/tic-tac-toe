use crate::kernel::game::{Action, ActionQueue};
use crate::State;
use std::cell::RefCell;
use std::rc::Rc;

mod tests;

#[derive(Debug)]
pub struct Random {
    state: Rc<RefCell<State>>,
}

impl Random {
    pub fn new(state: Rc<RefCell<State>>) -> Random {
        Self { state }
    }
}

impl ActionQueue for Random {
    fn pop(&self) -> Option<Action> {
        todo!()
    }
}
