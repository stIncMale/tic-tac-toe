pub struct World {
    state: State,
    system: MultiRoundGame,
}

impl World {
    pub fn new() -> World {
        World {
            state: State::new(),
            system: MultiRoundGame::new(),
        }
    }
}

trait System {
    fn advance(world: &mut State, input: &Input);
}

struct MultiRoundGame {}

impl MultiRoundGame {
    fn new() -> MultiRoundGame {
        MultiRoundGame {}
    }
}

impl System for MultiRoundGame {
    fn advance(state: &mut State, input: &Input) {
        println!("{:#?}, {:#?}", state, input);
    }
}

#[derive(Debug)]
pub struct State {
    players: [Player; 2],
    turn: u32,
    field: [[Option<Mark>; 3]; 3],
}

impl State {
    pub fn new() -> State {
        State {
            field: [[Option::None; 3]; 3],
            players: [Player::new(Mark::O), Player::new(Mark::X)],
            turn: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub mark: Mark,
    pub score: u32,
}

impl Player {
    fn new(mark: Mark) -> Player {
        Player { mark, score: 0 }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Mark {
    O,
    X,
}

#[derive(Debug, Copy, Clone)]
pub enum Input {
    Occupy(Cell),
    Surrender,
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    x: u32,
    y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell { x, y }
    }
}
