#![cfg(test)]
#![allow(non_snake_case)]

use crate::game::Phase::Inround;
use crate::game::{Action, ActionQueue, Board};
use crate::Mark::{O, X};
use crate::{Mark, Player, PlayerId, State};
use std::cell::RefCell;
use std::collections::HashSet;

mod Logic_single_action {
    use crate::game::game_tests::{
        player_set_wins, required_ready_from_players, state_with_board, VecActionQueue,
    };
    use crate::game::Action::{Occupy, Ready, Surrender};
    use crate::game::Phase::{Beginning, Inround, Outround};
    use crate::game::{ActionQueue, Board, Cell};
    use crate::Logic;
    use crate::Mark::{O, X};
    use pretty_assertions_sorted::assert_eq_sorted;
    use std::rc::Rc;
    use test_case::test_case;

    #[test]
    fn advance__no_action() {
        let mut state = state_with_board(Board {
            cells: [
                [None, None, None],
                [None, None, Some(X)],
                [None, None, None],
            ],
        });
        Logic::new([
            Rc::new(VecActionQueue::new(state.players[0].id, vec![])),
            Rc::new(VecActionQueue::new(state.players[1].id, vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(
            state,
            state_with_board(Board {
                cells: [
                    [None, None, None],
                    [None, None, Some(X)],
                    [None, None, None]
                ]
            })
        );
    }

    #[test]
    fn advance__occupy_action() {
        let mut state = state_with_board(Board::new());
        Logic::new([
            Rc::new(VecActionQueue::new(
                state.players[0].id,
                vec![Some(Occupy(Cell::new(1, 2)))],
            )),
            Rc::new(VecActionQueue::new(state.players[1].id, vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [None, None, None],
                    [None, None, Some(X)],
                    [None, None, None],
                ],
            });
            expected_state.step = 1;
            expected_state
        });
    }

    #[test]
    fn advance__surrender_action() {
        let mut state = state_with_board(Board::new());
        let expected_required_ready = required_ready_from_players(&state.players);
        Logic::new([
            Rc::new(VecActionQueue::new(
                state.players[0].id,
                vec![Some(Surrender)],
            )),
            Rc::new(VecActionQueue::new(state.players[1].id, vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board::new());
            player_set_wins(&mut expected_state, O, 1);
            expected_state.phase = Outround;
            expected_state.required_ready = expected_required_ready;
            expected_state
        });
    }

    #[test_case(
        &Board { cells: [[Some(X), Some(X), Some(X)], [None, None, None], [None, None, None]] },
        &Cell::new(0, 0), true)]
    #[test_case(
        &Board { cells: [[None, None, None], [Some(X), Some(X), Some(X)], [None, None, None]] },
        &Cell::new(1, 0), true)]
    #[test_case(
        &Board { cells: [[None, None, None], [None, None, None], [Some(X), Some(X), Some(X)]] },
        &Cell::new(2, 0), true)]
    #[test_case(
        &Board { cells: [[Some(X), None, None], [Some(X), None, None], [Some(X), None, None]] },
        &Cell::new(1, 0), true)]
    #[test_case(
        &Board { cells: [[None, Some(X), None], [None, Some(X), None], [None, Some(X), None]] },
        &Cell::new(1, 1), true)]
    #[test_case(
        &Board { cells: [[None, None, Some(X)], [None, None, Some(X)], [None, None, Some(X)]] },
        &Cell::new(1, 2), true)]
    #[test_case(
        &Board { cells: [[Some(X), None, None], [None, Some(X), None], [None, None, Some(X)]] },
        &Cell::new(1, 1), true)]
    #[test_case(
        &Board { cells: [[None, None, Some(X)], [None, Some(X), None], [Some(X), None, None]] },
        &Cell::new(1, 1), true)]
    #[test_case(
        &Board { cells: [[Some(X), Some(X), None], [Some(X), None, None], [None, None, None]] },
        &Cell::new(1, 0), false)]
    #[test_case(
        &Board { cells: [[Some(X), None, None], [None, None, Some(X)], [None, Some(X), None]] },
        &Cell::new(0, 0), false)]
    #[test_case(
        &Board { cells: [[None, Some(X), None], [None, None, Some(X)], [None, Some(X), None]] },
        &Cell::new(0, 1), false)]
    #[test_case(
        &Board { cells: [[Some(X), Some(O), Some(X)], [None, None, None], [None, None, None]] },
        &Cell::new(0, 2), false)]
    fn is_win(board: &Board, last_occupied: &Cell, expected: bool) {
        pretty_assertions_sorted::assert_eq!(Logic::is_win(board, last_occupied), expected);
    }

    #[test]
    fn advance__win() {
        let mut state = {
            let mut state = state_with_board(Board {
                cells: [
                    [Some(X), None, None],
                    [Some(O), Some(X), Some(O)],
                    [None, None, None],
                ],
            });
            state.step = 4;
            state
        };
        let expected_required_ready = required_ready_from_players(&state.players);
        Logic::new([
            Rc::new(VecActionQueue::new(
                state.players[0].id,
                vec![Some(Occupy(Cell::new(2, 2)))],
            )),
            Rc::new(VecActionQueue::new(state.players[1].id, vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [Some(X), None, None],
                    [Some(O), Some(X), Some(O)],
                    [None, None, Some(X)],
                ],
            });
            player_set_wins(&mut expected_state, X, 1);
            expected_state.phase = Outround;
            expected_state.step = 4;
            expected_state.required_ready = expected_required_ready;
            expected_state
        });
    }

    #[test]
    fn advance__draw() {
        let mut state = {
            let mut state = state_with_board(Board {
                cells: [
                    [Some(O), Some(X), Some(O)],
                    [Some(O), Some(X), Some(X)],
                    [Some(X), Some(O), None],
                ],
            });
            state.step = 8;
            state
        };
        let expected_required_ready = required_ready_from_players(&state.players);
        Logic::new([
            Rc::new(VecActionQueue::new(
                state.players[0].id,
                vec![Some(Occupy(Cell::new(2, 2)))],
            )),
            Rc::new(VecActionQueue::new(state.players[1].id, vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [Some(O), Some(X), Some(O)],
                    [Some(O), Some(X), Some(X)],
                    [Some(X), Some(O), Some(X)],
                ],
            });
            expected_state.phase = Outround;
            expected_state.step = 8;
            expected_state.required_ready = expected_required_ready;
            expected_state
        });
    }

    #[test]
    fn advance__ready_action__outround() {
        let mut state = {
            let mut state = state_with_board(Board {
                cells: [
                    [Some(X), Some(O), Some(X)],
                    [None, Some(X), Some(O)],
                    [Some(X), None, Some(O)],
                ],
            });
            player_set_wins(&mut state, X, 1);
            state.phase = Outround;
            state.step = 6;
            state.required_ready = [state.players[0].id].into_iter().collect();
            state
        };
        Logic::new([
            Rc::new(VecActionQueue::new(state.players[0].id, vec![Some(Ready)])),
            Rc::new(VecActionQueue::new(state.players[1].id, vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board::new());
            player_set_wins(&mut expected_state, X, 1);
            expected_state.phase = Inround;
            expected_state.round = 1;
            expected_state.step = 0;
            expected_state
        });
    }

    #[test]
    fn advance__ready_action__beginning() {
        let mut state = {
            let mut state = state_with_board(Board::new());
            state.phase = Beginning;
            state.required_ready = [state.players[0].id].into_iter().collect();
            state
        };
        Logic::new([
            Rc::new(VecActionQueue::new(state.players[0].id, vec![Some(Ready)])),
            Rc::new(VecActionQueue::new(state.players[1].id, vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board::new());
            expected_state.phase = Inround;
            expected_state.round = 0;
            expected_state.step = 0;
            expected_state
        });
    }

    #[test]
    fn advance__stop_at_phase_change() {
        let mut state = state_with_board(Board::new());
        let expected_required_ready = required_ready_from_players(&state.players);
        let act_queue_px = Rc::new(VecActionQueue::new(
            state.players[0].id,
            vec![Some(Surrender), Some(Occupy(Cell::new(0, 0)))],
        ));
        Logic::new([
            Rc::clone(&act_queue_px) as Rc<dyn ActionQueue>,
            Rc::new(VecActionQueue::new(state.players[1].id, vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board::new());
            player_set_wins(&mut expected_state, O, 1);
            expected_state.phase = Outround;
            expected_state.required_ready = expected_required_ready;
            expected_state
        });
        assert_ne!(act_queue_px.pop(), None);
    }
}

mod Logic_multiple_actions {
    use crate::game::game_tests::{
        player_set_wins, required_ready_from_players, state_with_board, VecActionQueue,
    };
    use crate::game::Action::{Occupy, Ready};
    use crate::game::Mark::{O, X};
    use crate::game::Phase::{Beginning, Outround};
    use crate::game::{Board, Cell};
    use crate::{ActionQueue, Logic};
    use pretty_assertions_sorted::assert_eq_sorted;
    use std::rc::Rc;

    #[test]
    fn win() {
        let mut state = {
            let mut state = state_with_board(Board::new());
            state.phase = Beginning;
            state.required_ready = required_ready_from_players(&state.players);
            state
        };
        let expected_required_ready = required_ready_from_players(&state.players);
        let act_queue_px = Rc::new(VecActionQueue::new(
            state.players[0].id,
            vec![
                None,
                None,
                Some(Ready),
                Some(Occupy(Cell::new(1, 1))),
                None,
                Some(Occupy(Cell::new(0, 0))),
                Some(Occupy(Cell::new(0, 2))),
                Some(Occupy(Cell::new(2, 0))),
            ],
        ));
        let act_queue_po = Rc::new(VecActionQueue::new(
            state.players[1].id,
            vec![
                Some(Ready),
                Some(Occupy(Cell::new(1, 2))),
                Some(Occupy(Cell::new(2, 2))),
                None,
                Some(Occupy(Cell::new(0, 1))),
            ],
        ));
        let actions_cnt = act_queue_px.actions.borrow().len() + act_queue_po.actions.borrow().len();
        let logic = Logic::new([act_queue_px, act_queue_po]);
        for _ in 0..actions_cnt {
            logic.advance(&mut state);
        }
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [Some(X), Some(O), Some(X)],
                    [None, Some(X), Some(O)],
                    [Some(X), None, Some(O)],
                ],
            });
            player_set_wins(&mut expected_state, X, 1);
            expected_state.phase = Outround;
            expected_state.round = 0;
            expected_state.step = 6;
            expected_state.required_ready = expected_required_ready;
            expected_state
        });
    }

    #[test]
    fn draw() {
        let mut state = {
            let mut state = state_with_board(Board::new());
            state.round = 1;
            state
        };
        let expected_required_ready = required_ready_from_players(&state.players);
        let act_queue_px = Rc::new(VecActionQueue::new(
            state.players[0].id,
            vec![
                Some(Occupy(Cell::new(0, 0))),
                Some(Occupy(Cell::new(1, 0))),
                Some(Occupy(Cell::new(0, 2))),
                Some(Occupy(Cell::new(2, 1))),
            ],
        ));
        let act_queue_po = Rc::new(VecActionQueue::new(
            state.players[1].id,
            vec![
                Some(Occupy(Cell::new(1, 1))),
                Some(Occupy(Cell::new(1, 2))),
                Some(Occupy(Cell::new(2, 0))),
                Some(Occupy(Cell::new(0, 1))),
                Some(Occupy(Cell::new(2, 2))),
            ],
        ));
        let logic = Logic::new([
            Rc::clone(&act_queue_px) as Rc<dyn ActionQueue>,
            Rc::clone(&act_queue_po) as Rc<dyn ActionQueue>,
        ]);
        let actions_cnt = act_queue_px.actions.borrow().len() + act_queue_po.actions.borrow().len();
        for _ in 0..actions_cnt {
            logic.advance(&mut state);
        }
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [Some(X), Some(O), Some(X)],
                    [Some(X), Some(O), Some(O)],
                    [Some(O), Some(X), Some(O)],
                ],
            });
            expected_state.phase = Outround;
            expected_state.round = 1;
            expected_state.step = 8;
            expected_state.required_ready = expected_required_ready;
            expected_state
        });
    }
}

mod DefaultActionQueue {
    use crate::game::Action::{Ready, Surrender};
    use crate::game::ActionQueue;
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

#[derive(Debug)]
struct VecActionQueue {
    player_id: PlayerId,
    actions: RefCell<Vec<Option<Action>>>,
}

impl VecActionQueue {
    fn new(player_id: PlayerId, mut actions: Vec<Option<Action>>) -> Self {
        actions.reverse();
        Self {
            player_id,
            actions: RefCell::new(actions),
        }
    }
}

impl ActionQueue for VecActionQueue {
    fn player_id(&self) -> PlayerId {
        self.player_id
    }

    fn pop(&self) -> Option<Action> {
        self.actions.borrow_mut().pop().unwrap_or(None)
    }
}

fn state_with_board(board: Board) -> State {
    State {
        board,
        players: [
            Player::new(PlayerId::new(0), X),
            Player::new(PlayerId::new(1), O),
        ],
        phase: Inround,
        rounds: State::DEFAULT_ROUNDS,
        round: 0,
        step: 0,
        required_ready: HashSet::new(),
    }
}

fn required_ready_from_players(players: &[Player]) -> HashSet<PlayerId> {
    players.iter().map(|p| p.id).collect::<HashSet<PlayerId>>()
}

fn player_set_wins(state: &mut State, player_mark: Mark, wins: u32) {
    state
        .players
        .iter_mut()
        .find(|p| p.mark == player_mark)
        .unwrap()
        .wins = wins;
}
