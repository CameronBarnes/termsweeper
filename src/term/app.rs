use crate::types::{Difficulty, Board};

pub struct App {
    pub should_quit: bool,
    board: Board,
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
        Self{should_quit: false, board: Board::new(difficulty)}
    }
}
