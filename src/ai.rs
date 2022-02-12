use crate::kernel::game as g;
use std::cell::RefCell;
use std::rc::Rc;

mod tests;

#[derive(Debug)]
pub struct Random {
    state: Rc<RefCell<g::State>>,
}

impl Random {
    pub fn new(state: Rc<RefCell<g::State>>) -> Random {
        Self { state }
    }
}

impl g::ActionQueue for Random {
    fn pop(&self) -> Option<g::Action> {
        todo!()
    }
}
