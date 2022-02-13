use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;

mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mark {
    X,
    O,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    id: PlayerId,
    mark: Mark,
    wins: u32,
}

impl Player {
    pub fn new(id: PlayerId, mark: Mark) -> Self {
        Self { id, mark, wins: 0 }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlayerId {
    idx: usize,
}

impl PlayerId {
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Phase {
    Beginning,
    Inround,
    Outround,
}

impl Default for Phase {
    fn default() -> Self {
        Phase::Beginning
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cell {
    x: usize,
    y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        assert!(x < Board::SIZE, "{:?}, {:?}", x, Board::SIZE);
        assert!(y < Board::SIZE, "{:?}, {:?}", y, Board::SIZE);
        Self { x, y }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Board {
    cells: [[Option<Mark>; Board::SIZE]; Board::SIZE],
}

impl Board {
    const SIZE: usize = 3;

    fn new() -> Self {
        Self::default()
    }

    fn set(&mut self, cell: &Cell, mark: Mark) {
        assert_eq!(self.cells[cell.x][cell.y], Option::None);
        self.cells[cell.x][cell.y] = Option::from(mark);
    }

    fn get(&self, cell: &Cell) -> Option<Mark> {
        self.cells[cell.x][cell.y]
    }

    fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Option::None;
            }
        }
    }

    fn size(&self) -> usize {
        self.cells.len()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct State {
    board: Board,
    players: [Player; State::PLAYER_COUNT],
    phase: Phase,
    game_rounds: u32,
    round: u32,
    step: u32,
    required_ready: HashSet<PlayerId>,
}

impl State {
    const PLAYER_COUNT: usize = 2;

    pub fn new(players: [Player; State::PLAYER_COUNT], game_rounds: u32) -> Self {
        for (idx, player) in players.iter().enumerate() {
            assert_eq!(player.id.idx, idx);
        }
        let required_ready = players.iter().map(|p| p.id).collect::<HashSet<PlayerId>>();
        Self {
            board: Board::new(),
            players,
            phase: Phase::default(),
            game_rounds,
            round: 0,
            step: 0,
            required_ready,
        }
    }

    fn turn(&self) -> PlayerId {
        PlayerId::new(usize::try_from(self.step + self.round).unwrap() % self.players.len())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    Ready,
    Occupy(Cell),
    Surrender,
}

pub trait ActionQueue: Debug {
    fn pop(&self) -> Option<Action>;
}

#[derive(Debug)]
pub struct Logic<'a> {
    action_queues: [&'a dyn ActionQueue; State::PLAYER_COUNT],
}

impl<'a> Logic<'a> {
    pub fn new(action_queues: [&'a dyn ActionQueue; State::PLAYER_COUNT]) -> Self {
        Self { action_queues }
    }

    fn advance(&self, state: &mut State) {
        match state.phase {
            Phase::Beginning | Phase::Outround => self.advance_beginning_outround(state),
            Phase::Inround => self.advance_inround(state),
        };
    }

    fn advance_beginning_outround(&self, state: &mut State) {
        for i in 0..state.players.len() {
            let player_id = state.players[i].id;
            if state.required_ready.contains(&player_id) {
                if let Option::Some(action) = self.action_queues[player_id.idx].pop() {
                    if action == Action::Ready {
                        Logic::ready(state, player_id);
                    } else {
                        panic!("{:?}, {:?}", player_id, action)
                    }
                }
            };
        }
    }

    fn advance_inround(&self, state: &mut State) {
        let player_id = state.turn();
        while let Option::Some(action) = self.action_queues[player_id.idx].pop() {
            match action {
                Action::Surrender => Logic::surrender(state),
                Action::Occupy(cell) => Logic::occupy(state, &cell),
                Action::Ready => panic!("{:?}, {:?}", player_id, action),
            }
            if state.turn() != player_id {
                break;
            }
        }
    }

    fn ready(state: &mut State, player_id: PlayerId) {
        assert!(
            state.phase == Phase::Beginning || state.phase == Phase::Outround,
            "{:?}, {:?}",
            state.phase,
            player_id
        );
        state.required_ready.remove(&player_id);
        if state.required_ready.is_empty() {
            match state.phase {
                Phase::Beginning => {}
                Phase::Outround => {
                    state.step = 0;
                    state.round += 1;
                    state.board.clear();
                }
                Phase::Inround => panic!(),
            }
            state.phase = Phase::Inround;
        }
    }

    fn surrender(state: &mut State) {
        assert_eq!(state.phase, Phase::Inround);
        // for more players this method would have been implemented quite differently
        assert_eq!(State::PLAYER_COUNT, 2);
        let idx_other_player = (state.turn().idx + 1) % state.players.len();
        state.players[idx_other_player].wins += 1;
        Logic::set_outround(state);
    }

    fn occupy(state: &mut State, cell: &Cell) {
        assert_eq!(state.phase, Phase::Inround);
        state.board.set(cell, state.players[state.turn().idx].mark);
        if Logic::win_condition(&state.board, cell) {
            Logic::win(state);
        } else if Logic::last_step(state.step, &state.board) {
            Logic::draw(state);
        } else {
            state.step += 1;
        }
    }

    fn set_outround(state: &mut State) {
        state.phase = Phase::Outround;
        state
            .required_ready
            .extend(state.players.iter().map(|p| p.id));
    }

    fn win_condition(board: &Board, last_occupied: &Cell) -> bool {
        let mut h_match = 0;
        let mut v_match = 0;
        let mut d1_match = 0;
        let mut d2_match = 0;
        let mark = board.get(last_occupied);
        assert_ne!(mark, Option::None);
        let Cell { x, y } = *last_occupied;
        let size = board.size();
        for i in 0..size {
            if board.get(&Cell::new(i, y)) == mark {
                h_match += 1;
            }
            if board.get(&Cell::new(x, i)) == mark {
                v_match += 1;
            }
            if board.get(&Cell::new(i, i)) == mark {
                d1_match += 1;
            }
            if board.get(&Cell::new(i, size - 1 - i)) == mark {
                d2_match += 1;
            }
        }
        h_match == size || v_match == size || d1_match == size || d2_match == size
    }

    fn last_step(step: u32, board: &Board) -> bool {
        step == u32::try_from(board.size().pow(2) - 1).unwrap()
    }

    fn win(state: &mut State) {
        state.players[state.turn().idx].wins += 1;
        Logic::set_outround(state);
    }

    fn draw(state: &mut State) {
        Logic::set_outround(state);
    }
}

#[derive(Debug)]
pub struct World<'a> {
    state: Rc<RefCell<State>>,
    logic: Logic<'a>,
}

impl<'a> World<'a> {
    pub fn new(state: Rc<RefCell<State>>, logic: Logic<'a>) -> Self {
        Self { state, logic }
    }

    pub fn advance(&self) {
        self.logic.advance(&mut *self.state.borrow_mut());
    }
}
