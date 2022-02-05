#![cfg(test)]
#![allow(dead_code, non_snake_case)]

use crate::core::game::{Action, ActionQueue, Board, Logic, Mark, Player, PlayerId, State};
use Mark::{O, X};

#[derive(Debug)]
struct VecActionQueue {
    actions: Vec<Action>,
}

impl VecActionQueue {
    fn new(mut actions: Vec<Action>) -> Self {
        actions.reverse();
        Self { actions }
    }
}

impl ActionQueue for VecActionQueue {
    fn next(&mut self) -> Option<Action> {
        self.actions.pop()
    }
}

fn state_with_board(board: Board) -> State {
    let mut state = State::new(
        [
            Player::new(PlayerId::new(0), X),
            Player::new(PlayerId::new(1), O),
        ],
        5,
    );
    state.board = board;
    state
}

fn board(cells: [[Option<Mark>; Board::SIZE]; Board::SIZE]) -> Board {
    let mut board = Board::new();
    board.cells = cells;
    board
}

fn player_with_wins(mut player: Player, wins: u32) -> Player {
    player.wins = wins;
    player
}

mod logic {
    use crate::core::game::tests::*;

    #[test]
    fn test_advance__none_action() {
        let mut state = state_with_board(board([
            [None, None, None],
            [None, None, Some(X)],
            [None, None, None],
        ]));
        Logic::new([
            &mut VecActionQueue::new(vec![]),
            &mut VecActionQueue::new(vec![]),
        ])
        .advance(&mut state);
        assert_eq!(
            state,
            state_with_board(board([
                [None, None, None],
                [None, None, Some(X)],
                [None, None, None]
            ]))
        );
    }
}
