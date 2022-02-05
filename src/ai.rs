use crate::core::game as g;

mod tests;

#[derive(Debug)]
pub struct Random<'a> {
    state: &'a g::State,
}

impl<'a> Random<'a> {
    pub fn new(state: &'a g::State) -> Random<'a> {
        Self { state }
    }
}

impl g::ActionQueue for Random<'_> {
    fn next(&mut self) -> Option<g::Action> {
        todo!()
    }
}
