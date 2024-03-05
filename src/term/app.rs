use crate::types::{Difficulty, Board};

pub struct App {
    pub should_quit: bool,
    pub control: bool,
    pub board: Board,
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
        Self{should_quit: false, control: false, board: Board::new(difficulty)}
    }
}
