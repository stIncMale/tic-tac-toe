#![cfg(test)]
#![allow(non_snake_case)]

mod Random {
    use crate::{ai, DefaultActionQueue, Logic, Mark, Player, PlayerId, State, World};
    use ntest::timeout;
    use oorandom::Rand64;
    use std::cell::RefCell;
    use std::panic;
    use std::rc::Rc;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    #[timeout(100)]
    fn play_against_itself() {
        let mut rng = Rand64::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        for _ in 0..1_000 {
            let ai_rng_seed_p0 = rng.rand_u64();
            let ai_rng_seed_p1 = rng.rand_u64();
            let p0 = Player::new(PlayerId::new(0), Mark::X);
            let p1 = Player::new(PlayerId::new(1), Mark::O);
            let p0_id = p0.id;
            let p1_id = p1.id;
            let act_queue_p0 = Rc::new(DefaultActionQueue::new(p0_id));
            let act_queue_p1 = Rc::new(DefaultActionQueue::new(p1_id));
            let state = Rc::new(RefCell::new(State::new([p0, p1], State::DEFAULT_ROUNDS)));
            let mut world = World::new(
                Rc::clone(&state),
                Logic::new([
                    Rc::clone(&act_queue_p0) as Rc<DefaultActionQueue>,
                    Rc::clone(&act_queue_p1) as Rc<DefaultActionQueue>,
                ]),
                vec![
                    Box::new(ai::Random::new(p0_id, ai_rng_seed_p0, act_queue_p0)),
                    Box::new(ai::Random::new(p1_id, ai_rng_seed_p1, act_queue_p1)),
                ],
            );
            let enough_iterations = {
                let state = state.borrow();
                u32::try_from(state.board.size().pow(2) + 1).unwrap() * state.rounds
            };
            for _ in 0..enough_iterations {
                world.advance();
            }
            assert!(
                Logic::<DefaultActionQueue>::is_game_over(&state.borrow()),
                "RNG seeds {:?}, {:?}",
                (ai_rng_seed_p0, ai_rng_seed_p1),
                &state.borrow()
            );
        }
    }
}
