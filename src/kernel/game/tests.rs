#![cfg(test)]
#![allow(non_snake_case)]

use crate::kernel::game::Phase::Inround;
use crate::kernel::game::{Action, ActionQueue, Board};
use crate::Mark::{O, X};
use crate::{Mark, Player, PlayerId, State};
use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Debug)]
struct VecActionQueue {
    actions: RefCell<Vec<Option<Action>>>,
}

impl VecActionQueue {
    fn new(mut actions: Vec<Option<Action>>) -> Self {
        actions.reverse();
        Self {
            actions: RefCell::new(actions),
        }
    }
}

impl ActionQueue for VecActionQueue {
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
        game_rounds: 5,
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

mod Logic_single_action {
    use crate::kernel::game::tests::{
        player_set_wins, required_ready_from_players, state_with_board, VecActionQueue,
    };
    use crate::kernel::game::Action::{Occupy, Ready, Surrender};
    use crate::kernel::game::Phase::{Beginning, Inround, Outround};
    use crate::kernel::game::{Board, Cell};
    use crate::Mark::{O, X};
    use crate::{Logic, PlayerId};
    use pretty_assertions_sorted::assert_eq_sorted;
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
        Logic::new([&VecActionQueue::new(vec![]), &VecActionQueue::new(vec![])])
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
            &VecActionQueue::new(vec![Some(Occupy(Cell::new(1, 2)))]),
            &VecActionQueue::new(vec![]),
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
            &VecActionQueue::new(vec![Some(Surrender)]),
            &VecActionQueue::new(vec![]),
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
    fn win_condition(board: &Board, last_occupied: &Cell, expected: bool) {
        assert_eq_sorted!(Logic::win_condition(board, last_occupied), expected);
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
            &VecActionQueue::new(vec![Some(Occupy(Cell::new(2, 2)))]),
            &VecActionQueue::new(vec![]),
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
            &VecActionQueue::new(vec![Some(Occupy(Cell::new(2, 2)))]),
            &VecActionQueue::new(vec![]),
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
            state.required_ready = [PlayerId::new(0)].into_iter().collect();
            state
        };
        Logic::new([
            &VecActionQueue::new(vec![Some(Ready)]),
            &VecActionQueue::new(vec![]),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut state = state_with_board(Board::new());
            player_set_wins(&mut state, X, 1);
            state.phase = Inround;
            state.round = 1;
            state.step = 0;
            state
        });
    }

    #[test]
    fn advance__ready_action__beginning() {
        let mut state = {
            let mut state = state_with_board(Board::new());
            state.phase = Beginning;
            state.required_ready = [PlayerId::new(0)].into_iter().collect();
            state
        };
        Logic::new([
            &VecActionQueue::new(vec![Some(Ready)]),
            &VecActionQueue::new(vec![]),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut state = state_with_board(Board::new());
            state.phase = Inround;
            state.round = 0;
            state.step = 0;
            state
        });
    }
}

mod Logic_multiple_actions {
    use crate::kernel::game::tests::{
        player_set_wins, required_ready_from_players, state_with_board, VecActionQueue,
    };
    use crate::kernel::game::Action::{Occupy, Ready};
    use crate::kernel::game::Mark::{O, X};
    use crate::kernel::game::Phase::{Beginning, Outround};
    use crate::kernel::game::{Board, Cell};
    use crate::Logic;
    use pretty_assertions_sorted::assert_eq_sorted;

    #[test]
    fn win() {
        let mut state = {
            let mut state = state_with_board(Board::new());
            state.phase = Beginning;
            state.required_ready = required_ready_from_players(&state.players);
            state
        };
        let expected_required_ready = required_ready_from_players(&state.players);
        let act_queue_px = VecActionQueue::new(vec![
            None,
            None,
            Some(Ready),
            Some(Occupy(Cell::new(1, 1))),
            None,
            Some(Occupy(Cell::new(0, 0))),
            Some(Occupy(Cell::new(0, 2))),
            Some(Occupy(Cell::new(2, 0))),
        ]);
        let act_queue_po = VecActionQueue::new(vec![
            Some(Ready),
            Some(Occupy(Cell::new(1, 2))),
            Some(Occupy(Cell::new(2, 2))),
            None,
            Some(Occupy(Cell::new(0, 1))),
        ]);
        let actions_cnt = act_queue_px.actions.borrow().len() + act_queue_po.actions.borrow().len();
        let logic = Logic::new([&act_queue_px, &act_queue_po]);
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
        let act_queue_px = VecActionQueue::new(vec![
            Some(Occupy(Cell::new(0, 0))),
            Some(Occupy(Cell::new(1, 0))),
            Some(Occupy(Cell::new(0, 2))),
            Some(Occupy(Cell::new(2, 1))),
        ]);
        let act_queue_po = VecActionQueue::new(vec![
            Some(Occupy(Cell::new(1, 1))),
            Some(Occupy(Cell::new(1, 2))),
            Some(Occupy(Cell::new(2, 0))),
            Some(Occupy(Cell::new(0, 1))),
            Some(Occupy(Cell::new(2, 2))),
        ]);
        let logic = Logic::new([&act_queue_px, &act_queue_po]);
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
