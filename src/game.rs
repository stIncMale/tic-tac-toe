use crate::game::Action::{Occupy, Ready, Surrender};
use crate::game::Phase::{Beginning, Inround, Outround};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

mod game_tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mark {
    X,
    O,
}

impl Display for Mark {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mark = match self {
            Mark::X => "X",
            Mark::O => "O",
        };
        write!(f, "{}", mark)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    pub id: PlayerId,
    mark: Mark,
    pub wins: u32,
}

impl Player {
    pub fn new(id: PlayerId, mark: Mark) -> Self {
        Self { id, mark, wins: 0 }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlayerId {
    pub idx: usize,
}

impl PlayerId {
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Phase {
    Beginning,
    Inround,
    Outround,
}

impl Default for Phase {
    fn default() -> Self {
        Beginning
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
pub struct Board {
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

    pub fn get(&self, cell: &Cell) -> Option<Mark> {
        self.cells[cell.x][cell.y]
    }

    fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = Option::None;
            }
        }
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct State {
    pub board: Board,
    pub players: [Player; State::PLAYER_COUNT],
    pub phase: Phase,
    pub rounds: u32,
    pub round: u32,
    pub step: u32,
    pub required_ready: HashSet<PlayerId>,
}

impl State {
    pub const DEFAULT_ROUNDS: u32 = 5;
    const PLAYER_COUNT: usize = 2;

    pub fn new(players: [Player; State::PLAYER_COUNT], rounds: u32) -> Self {
        for (idx, player) in players.iter().enumerate() {
            assert_eq!(player.id.idx, idx);
        }
        let required_ready = players.iter().map(|p| p.id).collect::<HashSet<PlayerId>>();
        Self {
            board: Board::new(),
            players,
            phase: Phase::default(),
            rounds,
            round: 0,
            step: 0,
            required_ready,
        }
    }

    pub fn turn(&self) -> PlayerId {
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
    fn player_id(&self) -> PlayerId;

    fn pop(&self) -> Option<Action>;
}

#[derive(Debug)]
pub struct DefaultActionQueue {
    player_id: PlayerId,
    actions: RefCell<VecDeque<Action>>,
}

impl DefaultActionQueue {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            actions: RefCell::new(VecDeque::with_capacity(1)),
        }
    }

    pub fn add(&self, action: Action) {
        self.actions.borrow_mut().push_back(action);
    }
}

impl ActionQueue for DefaultActionQueue {
    fn player_id(&self) -> PlayerId {
        self.player_id
    }

    fn pop(&self) -> Option<Action> {
        self.actions.borrow_mut().pop_front()
    }
}

#[derive(Debug)]
pub struct Logic {
    action_queues: [Rc<dyn ActionQueue>; State::PLAYER_COUNT],
}

impl Logic {
    pub fn new(action_queues: [Rc<dyn ActionQueue>; State::PLAYER_COUNT]) -> Self {
        for (idx, action_queue) in action_queues.iter().enumerate() {
            assert_eq!(action_queue.player_id().idx, idx);
        }
        Self { action_queues }
    }

    fn advance(&self, state: &mut State) {
        match state.phase {
            Beginning | Outround => self.advance_beginning_outround(state),
            Inround => self.advance_inround(state),
        };
    }

    fn advance_beginning_outround(&self, state: &mut State) {
        for i in 0..state.players.len() {
            let player_id = state.players[i].id;
            if state.required_ready.contains(&player_id) {
                if let Option::Some(action) = self.action_queues[player_id.idx].pop() {
                    if Logic::is_game_over(state) {
                        panic!()
                    }
                    if action == Ready {
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
            if Logic::is_game_over(state) {
                panic!()
            }
            match action {
                Surrender => Logic::surrender(state),
                Occupy(cell) => Logic::occupy(state, &cell),
                Ready => panic!("{:?}, {:?}", player_id, action),
            }
            if state.turn() != player_id || state.phase != Inround {
                break;
            }
        }
    }

    fn ready(state: &mut State, player_id: PlayerId) {
        assert!(
            state.phase == Beginning || state.phase == Outround,
            "{:?}, {:?}",
            state.phase,
            player_id
        );
        state.required_ready.remove(&player_id);
        if state.required_ready.is_empty() {
            match state.phase {
                Beginning => {}
                Outround => {
                    state.step = 0;
                    state.round += 1;
                    state.board.clear();
                }
                Inround => panic!(),
            }
            state.phase = Inround;
        }
    }

    fn surrender(state: &mut State) {
        assert_eq!(state.phase, Inround);
        // for more players this method would have been implemented quite differently
        assert_eq!(State::PLAYER_COUNT, 2);
        let idx_other_player = (state.turn().idx + 1) % state.players.len();
        state.players[idx_other_player].wins += 1;
        Logic::end_round(state);
    }

    fn occupy(state: &mut State, cell: &Cell) {
        assert_eq!(state.phase, Inround);
        state.board.set(cell, state.players[state.turn().idx].mark);
        if Logic::is_win(&state.board, cell) {
            Logic::win(state);
        } else if Logic::last_step(state.step, &state.board) {
            Logic::draw(state);
        } else {
            state.step += 1;
        }
    }

    fn end_round(state: &mut State) {
        state.phase = Outround;
        if !Logic::is_game_over(state) {
            state
                .required_ready
                .extend(state.players.iter().map(|p| p.id));
        }
    }

    fn is_win(board: &Board, last_occupied: &Cell) -> bool {
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
        Logic::end_round(state);
    }

    fn draw(state: &mut State) {
        Logic::end_round(state);
    }

    pub fn is_game_over(state: &State) -> bool {
        state.round == state.rounds - 1 && state.phase == Phase::Outround
    }
}

pub trait Ai: Debug {
    fn act(&self, state: &State);
}

#[derive(Debug)]
pub struct World {
    state: Rc<RefCell<State>>,
    logic: Logic,
    ais: Vec<Box<dyn Ai>>,
}

impl World {
    pub fn new(state: Rc<RefCell<State>>, logic: Logic, ais: Vec<Box<dyn Ai>>) -> Self {
        Self { state, logic, ais }
    }

    pub fn advance(&self) {
        let state = &mut *self.state.borrow_mut();
        for ai in &self.ais {
            ai.act(state);
        }
        self.logic.advance(state);
    }
}
