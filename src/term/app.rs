use crate::types::{Difficulty, Board};

pub struct App {
    pub should_quit: bool,
    pub board: Board,
    pub double_click: Option<(u16, u16)>,
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
        Self{should_quit: false, board: Board::new(difficulty), double_click: None}
    }
}
