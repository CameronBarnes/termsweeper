use ratatui::{prelude::Rect, widgets::{Paragraph, Block, Borders}};

use crate::types::{Difficulty, Board};

pub struct App {
    pub should_quit: bool,
    board: Board,
    pub board_rect: Rect,
    last_click_pos: (usize, usize),
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
        Self{should_quit: false, board: Board::new(difficulty), board_rect: Rect::default(), last_click_pos: (0 ,0)}
    }

    const fn translate_click_coordinates(&self, x: usize, y: usize) -> (usize, usize) {
        let x = x.saturating_sub(self.board_rect.x as usize + 1).saturating_div(3);
        let y = y.saturating_sub(self.board_rect.y as usize + 1).saturating_div(3);
        (x, y)
    }

    pub fn left_click(&mut self, x: usize, y: usize) {
        let (x, y) = self.translate_click_coordinates(x, y);
        if self.last_click_pos.0 == x && self.last_click_pos.1 == y {
            self.board.do_control_click(x, y);
        } else {
            self.board.left_click(x, y);
            self.last_click_pos = (x, y);
        }
    }

    pub fn right_click(&mut self, x: usize, y: usize) {
        let (x, y) = self.translate_click_coordinates(x, y);
        self.board.right_click(x, y);
    }

    pub fn middle_click(&mut self, x: usize, y: usize) {
        let (x, y) = self.translate_click_coordinates(x, y);
        self.board.middle_click(x, y);
    }

    pub const fn difficulty(&self) -> Difficulty {
        self.board.difficulty
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.board.difficulty = difficulty;
    }

    pub fn get_board_widget(&self) -> Paragraph {
        let time = self.board.first_move_time().map_or_else(
            || String::from("0s"),
            |start| {
                self.board.last_move_time().map_or_else(
                    || format!("{:?}", start.elapsed()),
                    |end| format!("{:?}", end.duration_since(start)),
                    )
            },
        );
        self.board.to_widget().block(
        Block::new()
            .borders(Borders::ALL)
            .title(format!("Minesweeper: {} - {}", self.difficulty(), time)),
        )
    }

    pub fn get_board_size_with_border(&self) -> (u16, u16) {
        let (x, y) = self.board.get_board_size();
        #[allow(clippy::cast_possible_truncation)]
        (x as u16 * 3 + 1, y as u16 * 3 + 1)
    }
}
