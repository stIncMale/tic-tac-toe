extern crate alloc;

use alloc::{collections::VecDeque, rc::Rc};
use core::{
    cell::RefCell,
    fmt::{Debug, Display, Formatter, Result},
};
use std::collections::HashSet;

use crate::game::{
    Action::{Occupy, Ready, Surrender},
    Phase::{Beginning, Inround, Outround},
    PlayerType::{Local, _Remote},
};

mod game_tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mark {
    X,
    O,
}

impl Display for Mark {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str(match self {
            Mark::X => "X",
            Mark::O => "O",
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    pub id: PlayerId,
    pub typ: PlayerType,
    pub wins: u32,
}

impl Player {
    pub fn new(id: PlayerId, typ: PlayerType) -> Self {
        Self { id, typ, wins: 0 }
    }

    pub fn mark(&self) -> Mark {
        match self.id.idx {
            0 => Mark::X,
            1 => Mark::O,
            _ => panic!("{:?}", self.id),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}: {}", self.mark(), self.typ)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlayerId {
    // TODO implement Deref so that PlayerId can be used as if its usize
    pub idx: usize,
}

impl PlayerId {
    pub fn new(idx: usize) -> Self {
        assert!(idx < State::PLAYER_COUNT, "{:?}", idx);
        Self { idx }
    }
}

impl From<usize> for PlayerId {
    fn from(idx: usize) -> Self {
        Self::new(idx)
    }
}

impl PartialEq<usize> for PlayerId {
    fn eq(&self, other: &usize) -> bool {
        self.idx == *other
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlayerType {
    Local(LocalPlayerType),
    _Remote,
}

impl Display for PlayerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Local(subtype) => match subtype {
                LocalPlayerType::Human => "local player",
                LocalPlayerType::Ai => "bot",
            },
            _Remote => "remote player",
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LocalPlayerType {
    Human,
    Ai,
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
    /// # Panics
    ///
    /// If the either `x` or `y` is greater than or equal to [`Board::SIZE`].
    pub fn new(x: usize, y: usize) -> Self {
        assert!(x < Board::SIZE, "{:?}, {:?}", x, Board::SIZE);
        assert!(y < Board::SIZE, "{:?}, {:?}", y, Board::SIZE);
        Self { x, y }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Board {
    cells: [[Option<PlayerId>; Board::SIZE]; Board::SIZE],
}

impl Board {
    const SIZE: usize = 3;

    fn new() -> Self {
        Self::default()
    }

    fn set(&mut self, cell: &Cell, player_id: PlayerId) {
        assert_eq!(self.cells[cell.x][cell.y], None);
        self.cells[cell.x][cell.y] = Option::from(player_id);
    }

    pub fn get(&self, cell: &Cell) -> Option<PlayerId> {
        self.cells[cell.x][cell.y]
    }

    fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = None;
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
    pub const PLAYER_COUNT: usize = 2;

    /// # Panics
    ///
    /// If the index of an item in `players` is not equal to the corresponding [`PlayerId`].
    pub fn new(players: [Player; State::PLAYER_COUNT], rounds: u32) -> Self {
        for (idx, player) in players.iter().enumerate() {
            assert_eq!(player.id, idx, "{:?}, {:?}", player, idx);
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
        PlayerId::from(
            usize::try_from(self.step + self.round).expect("should fit") % self.players.len(),
        )
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

/// Instances of this struct need to be "owned" via [`Rc`]
/// by both the [`World`] and components that produce [`Action`]s.
/// TODO is interior mutability needed?
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
pub struct Logic<A> {
    action_queues: [Rc<A>; State::PLAYER_COUNT],
}

impl<A> Logic<A>
where
    A: ActionQueue,
{
    /// # Panics
    ///
    /// If the index of an item in `action_queues` is not equal to the corresponding [`PlayerId`].
    pub fn new(action_queues: [Rc<A>; State::PLAYER_COUNT]) -> Self {
        for (idx, action_queue) in action_queues.iter().enumerate() {
            assert_eq!(action_queue.player_id(), idx);
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
                if let Some(action) = self.action_queues[player_id.idx].pop() {
                    assert!(
                        !Logic::<A>::is_game_over(state),
                        "{:?}, {:?}, {:?}",
                        state,
                        player_id,
                        action
                    );
                    if action == Ready {
                        Logic::<A>::ready(state, player_id);
                    } else {
                        panic!("{:?}, {:?}, {:?}", state, player_id, action)
                    }
                }
            };
        }
    }

    fn advance_inround(&self, state: &mut State) {
        let player_id = state.turn();
        while let Some(action) = self.action_queues[player_id.idx].pop() {
            assert!(
                !Logic::<A>::is_game_over(state),
                "{:?}, {:?}",
                state,
                action
            );
            match action {
                Surrender => Logic::<A>::surrender(state),
                Occupy(cell) => Logic::<A>::occupy(state, &cell),
                Ready => panic!("{:?}, {:?}", state, action),
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
            state,
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
                Inround => panic!("{:?}, {:?}", state, player_id),
            }
            state.phase = Inround;
        }
    }

    fn surrender(state: &mut State) {
        assert_eq!(
            State::PLAYER_COUNT,
            2,
            "for more players this method would have been implemented quite differently"
        );
        assert_eq!(state.phase, Inround);
        let idx_other_player = (state.turn().idx + 1) % state.players.len();
        state.players[idx_other_player].wins += 1;
        Logic::<A>::end_round(state);
    }

    fn occupy(state: &mut State, cell: &Cell) {
        assert_eq!(state.phase, Inround);
        state.board.set(cell, state.turn());
        if Logic::<A>::is_win(&state.board, cell) {
            Logic::<A>::win(state);
        } else if Logic::<A>::last_step(state.step, &state.board) {
            Logic::<A>::draw(state);
        } else {
            state.step += 1;
        }
    }

    fn end_round(state: &mut State) {
        state.phase = Outround;
        if !Logic::<A>::is_game_over(state) {
            state
                .required_ready
                .extend(state.players.iter().map(|p| p.id));
        }
    }

    // TODO instead return Option<(Cell, Cell)> specifying the winning line
    // Then use it in TUI for fighlighting
    fn is_win(board: &Board, last_occupied: &Cell) -> bool {
        let mut h_match = 0;
        let mut v_match = 0;
        let mut d1_match = 0;
        let mut d2_match = 0;
        let mark = board.get(last_occupied);
        assert_ne!(mark, None);
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
        step == u32::try_from(board.size().pow(2) - 1).expect("should fit")
    }

    fn win(state: &mut State) {
        state.players[state.turn().idx].wins += 1;
        Logic::<A>::end_round(state);
    }

    fn draw(state: &mut State) {
        Logic::<A>::end_round(state);
    }

    pub fn is_game_over(state: &State) -> bool {
        state.round == state.rounds - 1 && state.phase == Outround
    }
}

pub trait Ai: Debug {
    fn player_id(&self) -> PlayerId;

    fn act(&mut self, state: &State);
}

#[derive(Debug)]
pub struct World<A> {
    state: State,
    logic: Logic<A>,
    ais: Vec<Box<dyn Ai>>,
}

impl<A> World<A>
where
    A: ActionQueue,
{
    pub fn new(state: State, logic: Logic<A>, ais: Vec<Box<dyn Ai>>) -> Self {
        assert!(
            !ais.iter()
                .any(|ai| state.players[ai.player_id().idx].typ != Local(LocalPlayerType::Ai)),
            "{:?}, {:?}",
            state,
            ais
        );
        Self { state, logic, ais }
    }

    pub fn advance(&mut self) {
        for ai in &mut self.ais {
            ai.act(&self.state);
        }
        self.logic.advance(&mut self.state);
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}
