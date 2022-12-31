#![cfg(test)]
#![allow(non_snake_case)]

use core::cell::RefCell;
use std::collections::HashSet;

use crate::{
    game::{Action, ActionQueue, Board, Phase::Inround},
    Human, Local, Player, PlayerId, State,
};

mod Line {
    use pretty_assertions_sorted::assert_eq;
    use test_case::test_case;

    use crate::game::{Cell, Line};

    #[test_case(Line::H(0), (1, 0).into(), true)]
    #[test_case(Line::H(0), (0, 1).into(), false)]
    #[test_case(Line::H(2), (0, 2).into(), true)]
    #[test_case(Line::H(2), (2, 0).into(), false)]
    #[test_case(Line::V(0), (0, 1).into(), true)]
    #[test_case(Line::V(0), (1, 0).into(), false)]
    #[test_case(Line::V(2), (2, 0).into(), true)]
    #[test_case(Line::V(2), (0, 2).into(), false)]
    #[test_case(Line::D1, (1, 1).into(), true)]
    #[test_case(Line::D1, (2, 0).into(), false)]
    #[test_case(Line::D2, (2, 0).into(), true)]
    #[test_case(Line::D2, (1, 1).into(), true)]
    fn contains(line: Line, cell: Cell, expected: bool) {
        assert_eq!(line.contains(&cell), expected);
    }
}

mod Logic_single_action {
    use alloc::rc::Rc;
    use std::collections::HashSet;

    use pretty_assertions_sorted::{assert_eq, assert_eq_sorted};
    use test_case::test_case;

    use crate::{
        game::{
            test::{required_ready_from_players, state_with_board, VecActionQueue},
            Action::{Occupy, Ready, Surrender},
            ActionQueue, Board, Cell, Line,
            Line::{D1, D2, H, V},
            Phase::{Beginning, Inround, Outround},
        },
        DefaultActionQueue, Logic, PlayerId,
    };

    #[test]
    fn advance__no_action() {
        let mut state = state_with_board(Board {
            cells: [
                [None, None, None],
                [None, None, Some(0.into())],
                [None, None, None],
            ],
        });
        Logic::new([
            Rc::new(VecActionQueue::new(PlayerId::new(0), vec![])),
            Rc::new(VecActionQueue::new(PlayerId::new(1), vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(
            state,
            state_with_board(Board {
                cells: [
                    [None, None, None],
                    [None, None, Some(0.into())],
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
                PlayerId::new(0),
                vec![Some(Occupy((1, 2).into()))],
            )),
            Rc::new(VecActionQueue::new(PlayerId::new(1), vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [None, None, None],
                    [None, None, Some(0.into())],
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
            Rc::new(VecActionQueue::new(PlayerId::new(0), vec![Some(Surrender)])),
            Rc::new(VecActionQueue::new(PlayerId::new(1), vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board::new());
            expected_state.players[1].wins = 1;
            expected_state.phase = Outround;
            expected_state.required_ready = expected_required_ready;
            expected_state
        });
    }

    #[test_case(
        &Board { cells: [
            [Some(0.into()), Some(0.into()), Some(0.into())],
            [None, None, None],
            [None, None, None]] },
        (0, 0).into(), Some(V(0)))]
    #[test_case(
        &Board { cells: [
            [None, None, None],
            [Some(0.into()), Some(0.into()), Some(0.into())],
            [None, None, None]] },
        (1, 0).into(), Some(V(1)))]
    #[test_case(
        &Board { cells: [
            [None, None, None],
            [None, None, None],
            [Some(0.into()), Some(0.into()), Some(0.into())]] },
        (2, 0).into(), Some(V(2)))]
    #[test_case(
        &Board { cells: [
            [Some(0.into()), None, None],
            [Some(0.into()), None, None],
            [Some(0.into()), None, None]] },
        (1, 0).into(), Some(H(0)))]
    #[test_case(
        &Board { cells: [
            [None, Some(0.into()), None],
            [None, Some(0.into()), None],
            [None, Some(0.into()), None]] },
        (1, 1).into(), Some(H(1)))]
    #[test_case(
        &Board { cells: [
            [None, None, Some(0.into())],
            [None, None, Some(0.into())],
            [None, None, Some(0.into())]] },
        (1, 2).into(), Some(H(2)))]
    #[test_case(
        &Board { cells: [
            [Some(0.into()), None, None],
            [None, Some(0.into()), None],
            [None, None, Some(0.into())]] },
        (1, 1).into(), Some(D1))]
    #[test_case(
        &Board { cells: [
            [None, None, Some(0.into())],
            [None, Some(0.into()), None],
            [Some(0.into()), None, None]] },
        (1, 1).into(), Some(D2))]
    #[test_case(
        &Board { cells: [
            [Some(0.into()), Some(0.into()), None],
            [Some(0.into()), None, None],
            [None, None, None]] },
        (1, 0).into(), None)]
    #[test_case(
        &Board { cells: [
            [Some(0.into()), None, None],
            [None, None, Some(0.into())],
            [None, Some(0.into()), None]] },
        (0, 0).into(), None)]
    #[test_case(
        &Board { cells: [
            [None, Some(0.into()), None],
            [None, None, Some(0.into())],
            [None, Some(0.into()), None]] },
        (0, 1).into(), None)]
    #[test_case(
        &Board { cells: [
            [Some(0.into()), Some(1.into()), Some(0.into())],
            [None, None, None],
            [None, None, None]] },
        (0, 2).into(), None)]
    fn check_win(board: &Board, last_occupied: Cell, expected: Option<Line>) {
        assert_eq!(
            Logic::<DefaultActionQueue>::check_win(board, &last_occupied),
            expected
        );
    }

    #[test]
    fn advance__win() {
        let mut state = {
            let mut state = state_with_board(Board {
                cells: [
                    [Some(0.into()), None, None],
                    [Some(1.into()), Some(0.into()), Some(1.into())],
                    [None, None, None],
                ],
            });
            state.step = 4;
            state
        };
        let expected_required_ready = required_ready_from_players(&state.players);
        Logic::new([
            Rc::new(VecActionQueue::new(
                PlayerId::new(0),
                vec![Some(Occupy((2, 2).into()))],
            )),
            Rc::new(VecActionQueue::new(PlayerId::new(1), vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [Some(0.into()), None, None],
                    [Some(1.into()), Some(0.into()), Some(1.into())],
                    [None, None, Some(0.into())],
                ],
            });
            expected_state.players[0].wins = 1;
            expected_state.phase = Outround;
            expected_state.step = 4;
            expected_state.required_ready = expected_required_ready;
            expected_state.win_line = Some(D1);
            expected_state
        });
    }

    #[test]
    fn advance__draw() {
        let mut state = {
            let mut state = state_with_board(Board {
                cells: [
                    [Some(1.into()), Some(0.into()), Some(1.into())],
                    [Some(1.into()), Some(0.into()), Some(0.into())],
                    [Some(0.into()), Some(1.into()), None],
                ],
            });
            state.step = 8;
            state
        };
        let expected_required_ready = required_ready_from_players(&state.players);
        Logic::new([
            Rc::new(VecActionQueue::new(
                PlayerId::new(0),
                vec![Some(Occupy((2, 2).into()))],
            )),
            Rc::new(VecActionQueue::new(PlayerId::new(1), vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [Some(1.into()), Some(0.into()), Some(1.into())],
                    [Some(1.into()), Some(0.into()), Some(0.into())],
                    [Some(0.into()), Some(1.into()), Some(0.into())],
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
                    [Some(0.into()), Some(1.into()), Some(0.into())],
                    [None, Some(0.into()), Some(1.into())],
                    [Some(0.into()), None, Some(1.into())],
                ],
            });
            state.players[0].wins = 1;
            state.phase = Outround;
            state.step = 6;
            state.required_ready = HashSet::from([PlayerId::new(0)]);
            state
        };
        Logic::new([
            Rc::new(VecActionQueue::new(PlayerId::new(0), vec![Some(Ready)])),
            Rc::new(VecActionQueue::new(PlayerId::new(1), vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board::new());
            expected_state.players[0].wins = 1;
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
            state.required_ready = HashSet::from([PlayerId::new(0)]);
            state
        };
        Logic::new([
            Rc::new(VecActionQueue::new(PlayerId::new(0), vec![Some(Ready)])),
            Rc::new(VecActionQueue::new(PlayerId::new(1), vec![])),
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
        let act_queue_p0 = Rc::new(VecActionQueue::new(
            PlayerId::new(0),
            vec![Some(Surrender), Some(Occupy((0, 0).into()))],
        ));
        Logic::new([
            Rc::clone(&act_queue_p0),
            Rc::new(VecActionQueue::new(PlayerId::new(1), vec![])),
        ])
        .advance(&mut state);
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board::new());
            expected_state.players[1].wins = 1;
            expected_state.phase = Outround;
            expected_state.required_ready = expected_required_ready;
            expected_state
        });
        assert_ne!(act_queue_p0.pop(), None);
    }
}

mod Logic_multiple_actions {
    use alloc::rc::Rc;

    use pretty_assertions_sorted::{assert_eq, assert_eq_sorted};

    use crate::{
        game::{
            test::{required_ready_from_players, state_with_board, VecActionQueue},
            Action::{Occupy, Ready},
            Board,
            Line::D2,
            Phase::{Beginning, Outround},
        },
        DefaultActionQueue, Logic, PlayerId,
    };

    #[test]
    fn win() {
        let mut state = {
            let mut state = state_with_board(Board::new());
            state.phase = Beginning;
            state.required_ready = required_ready_from_players(&state.players);
            state
        };
        let expected_required_ready = required_ready_from_players(&state.players);
        let act_queue_p0 = {
            let queue = DefaultActionQueue::new(PlayerId::new(0));
            queue.add(Ready);
            queue.add(Occupy((1, 1).into()));
            queue.add(Occupy((0, 0).into()));
            queue.add(Occupy((0, 2).into()));
            queue.add(Occupy((2, 0).into()));
            Rc::new(queue)
        };
        let act_queue_p1 = {
            let queue = DefaultActionQueue::new(PlayerId::new(1));
            queue.add(Ready);
            queue.add(Occupy((1, 2).into()));
            queue.add(Occupy((2, 2).into()));
            queue.add(Occupy((0, 1).into()));
            Rc::new(queue)
        };
        let actions_cnt = act_queue_p0.actions.borrow().len() + act_queue_p1.actions.borrow().len();
        let logic = Logic::new([Rc::clone(&act_queue_p0), Rc::clone(&act_queue_p1)]);
        for _ in 0..actions_cnt {
            logic.advance(&mut state);
        }
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [Some(0.into()), Some(1.into()), Some(0.into())],
                    [None, Some(0.into()), Some(1.into())],
                    [Some(0.into()), None, Some(1.into())],
                ],
            });
            expected_state.players[0].wins = 1;
            expected_state.phase = Outround;
            expected_state.round = 0;
            expected_state.step = 6;
            expected_state.required_ready = expected_required_ready;
            expected_state.win_line = Some(D2);
            expected_state
        });
        act_queue_p0.add(Ready);
        act_queue_p1.add(Ready);
        logic.advance(&mut state);
        assert_eq!(state.win_line, None);
    }

    #[test]
    fn draw() {
        let mut state = {
            let mut state = state_with_board(Board::new());
            state.round = 1;
            state
        };
        let expected_required_ready = required_ready_from_players(&state.players);
        let act_queue_p0 = Rc::new(VecActionQueue::new(
            PlayerId::new(0),
            vec![
                None,
                None,
                Some(Occupy((0, 0).into())),
                Some(Occupy((1, 0).into())),
                None,
                Some(Occupy((0, 2).into())),
                Some(Occupy((2, 1).into())),
            ],
        ));
        let act_queue_p1 = Rc::new(VecActionQueue::new(
            PlayerId::new(1),
            vec![
                Some(Occupy((1, 1).into())),
                Some(Occupy((1, 2).into())),
                Some(Occupy((2, 0).into())),
                None,
                None,
                Some(Occupy((0, 1).into())),
                Some(Occupy((2, 2).into())),
                None,
            ],
        ));
        let logic = Logic::new([Rc::clone(&act_queue_p0), Rc::clone(&act_queue_p1)]);
        let actions_cnt = act_queue_p0.actions.borrow().len() + act_queue_p1.actions.borrow().len();
        for _ in 0..actions_cnt {
            logic.advance(&mut state);
        }
        assert_eq_sorted!(state, {
            let mut expected_state = state_with_board(Board {
                cells: [
                    [Some(0.into()), Some(1.into()), Some(0.into())],
                    [Some(0.into()), Some(1.into()), Some(1.into())],
                    [Some(1.into()), Some(0.into()), Some(1.into())],
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
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        game::{
            Action::{Ready, Surrender},
            ActionQueue,
        },
        DefaultActionQueue, PlayerId,
    };

    #[test]
    fn add_pop() {
        let action_queue = DefaultActionQueue::new(PlayerId::new(0));
        assert_eq!(action_queue.pop(), None);
        action_queue.add(Ready);
        action_queue.add(Surrender);
        assert_eq!(action_queue.pop(), Some(Ready));
        assert_eq!(action_queue.pop(), Some(Surrender));
        assert_eq!(action_queue.pop(), None);
        assert_eq!(action_queue.pop(), None);
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
    let mut state = State::new(
        [
            Player::new(PlayerId::new(0), Local(Human)),
            Player::new(PlayerId::new(1), Local(Human)),
        ],
        State::DEFAULT_ROUNDS,
    );
    state.board = board;
    state.phase = Inround;
    state.required_ready.clear();
    state
}

fn required_ready_from_players(players: &[Player]) -> HashSet<PlayerId> {
    players.iter().map(|p| p.id).collect::<HashSet<PlayerId>>()
}
