#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State {
    pieces: [[Square; 3]; 3],
    turn: Turn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Turn {
    X,
    O,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty,
    X,
    O,
}
