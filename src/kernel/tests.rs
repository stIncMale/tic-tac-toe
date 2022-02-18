#![cfg(test)]
#![allow(non_snake_case)]

mod DefaultActionQueue {
    use crate::kernel::game::Action::{Ready, Surrender};
    use crate::kernel::game::ActionQueue;
    use crate::{DefaultActionQueue, PlayerId};
    use pretty_assertions_sorted::assert_eq_sorted;

    #[test]
    fn add_pop() {
        let action_queue = DefaultActionQueue::new(PlayerId::new(0));
        assert_eq_sorted!(action_queue.pop(), None);
        action_queue.add(Ready);
        action_queue.add(Surrender);
        assert_eq_sorted!(action_queue.pop(), Some(Ready));
        assert_eq_sorted!(action_queue.pop(), Some(Surrender));
        assert_eq_sorted!(action_queue.pop(), None);
        assert_eq_sorted!(action_queue.pop(), None);
    }
}
