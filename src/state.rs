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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    piece: Turn,
    location: Coordinate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}
impl From<Turn> for Square {
    fn from(item: Turn) -> Self {
        match item {
            Turn::X => Self::X,
            Turn::O => Self::O,
        }
    }
}

impl Turn {
    pub fn flip(&mut self) {
        match self {
            Turn::O => *self = Turn::X,
            Turn::X => *self = Turn::O,
        }
    }
}

impl State {
    /// Make a legal move on the board.
    pub fn make_move(&mut self, move_to_make: Move) {
        self.turn.flip();
        self.pieces[move_to_make.location.y][move_to_make.location.x] = move_to_make.piece.into();
    }
    /// Undo a given move on the board.
    pub fn undo_move(&mut self, move_to_undo: Move) {
        self.turn.flip();
        self.pieces[move_to_undo.location.y][move_to_undo.location.x] = move_to_undo.piece.into();
    }
    /// Checks if the game is over, if it is, then return the side who won. Otherwise return None.
    pub fn is_game_end(&self) -> Option<Turn> {
        todo!()
    }
    /// Returns moves for the current side to play.
    // TODO: Switch this to ArrayVec if that speeds this up.
    pub fn get_moves(&self) -> Vec<Move> {
        todo!()
    }
}
