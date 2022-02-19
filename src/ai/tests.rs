#![cfg(test)]
#![allow(non_snake_case)]

mod Random {
    use crate::{ai, ActionQueue, DefaultActionQueue, Logic, Mark, Player, PlayerId, State, World};
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
            let ai_rng_seed_px = rng.rand_u64();
            let ai_rng_seed_po = rng.rand_u64();
            let result = panic::catch_unwind(|| {
                let px = Player::new(PlayerId::new(0), Mark::X);
                let po = Player::new(PlayerId::new(1), Mark::O);
                let px_id = px.id;
                let po_id = po.id;
                let act_queue_px = Rc::new(DefaultActionQueue::new(px_id));
                let act_queue_po = Rc::new(DefaultActionQueue::new(po_id));
                let state = Rc::new(RefCell::new(State::new([px, po], 5)));
                let world = World::new(
                    Rc::clone(&state),
                    Logic::new([
                        Rc::clone(&act_queue_px) as Rc<dyn ActionQueue>,
                        Rc::clone(&act_queue_po) as Rc<dyn ActionQueue>,
                    ]),
                    vec![
                        Box::new(ai::Random::new(px_id, ai_rng_seed_px, act_queue_px)),
                        Box::new(ai::Random::new(po_id, ai_rng_seed_po, act_queue_po)),
                    ],
                );
                while !Logic::is_game_over(&state.borrow()) {
                    world.advance();
                }
                assert!(
                    Logic::is_game_over(&state.borrow()),
                    "{:?}",
                    &state.borrow()
                );
            });
            assert!(result.is_ok(), "{}, {}", ai_rng_seed_px, ai_rng_seed_po);
        }
    }
}
