use ratatui::{
    prelude::{Alignment, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, List, Paragraph},
};

use crate::{
    io::{read_leaderboard, write_leaderboard},
    types::{Board, Difficulty, Score, TileState},
};

pub struct App {
    pub should_quit: bool,
    board: Board,
    pub board_rect: Rect,
    last_click_pos: (usize, usize),
    leaderboard_updated: bool,
    leaderboard: Vec<Score>,
    pub change_difficulty: bool,
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
        let mut leaderboard = read_leaderboard().unwrap_or_default();
        leaderboard.sort_unstable_by_key(Score::time);
        Self {
            should_quit: false,
            board: Board::new(difficulty),
            board_rect: Rect::default(),
            last_click_pos: (0, 0),
            leaderboard_updated: false,
            leaderboard,
            change_difficulty: false,
        }
    }

    pub fn tick(&mut self) {
        if self.board.last_move_time().is_some() && !self.leaderboard_updated {
            if self.board.check_all_mine_state(TileState::Marked) {
                let time = self
                    .board
                    .last_move_time()
                    .unwrap()
                    .duration_since(self.board.first_move_time().unwrap());
                self.leaderboard
                    .push(Score::new(self.board.difficulty, time));
                self.leaderboard.sort_unstable_by_key(Score::time);
                self.leaderboard.dedup();
                let _ = write_leaderboard(&self.leaderboard);
            }
            self.leaderboard_updated = true;
        }

        if self.board.last_move_time().is_some()
            && !self.board.check_all_mine_state(TileState::Marked)
            && !self.board.check_all_mine_state(TileState::Visible)
        {
            self.board.do_game_over_animation();
        } else {
            self.board.clear_fire();
        }
    }

    fn translate_click_coordinates(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let x = x
            .checked_sub(self.board_rect.x as usize + 1)
            .and_then(|x| x.checked_div(3));
        let y = y
            .checked_sub(self.board_rect.y as usize + 1)
            .and_then(|y| y.checked_div(3));
        if let (Some(x), Some(y)) = (x, y) {
            let (board_x, board_y) = self.board.get_board_size();
            if x < board_x && y < board_y {
                Some((x, y))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn left_click(&mut self, x: usize, y: usize) {
        if self.change_difficulty {
            return;
        }
        if let Some((x, y)) = self.translate_click_coordinates(x, y) {
            if self.last_click_pos.0 == x && self.last_click_pos.1 == y {
                self.board.do_control_click(x, y);
            } else {
                self.board.left_click(x, y);
                self.last_click_pos = (x, y);
            }
        }
    }

    pub fn right_click(&mut self, x: usize, y: usize) {
        if self.change_difficulty {
            return;
        }
        if let Some((x, y)) = self.translate_click_coordinates(x, y) {
            self.board.right_click(x, y);
        }
    }

    pub fn middle_click(&mut self, x: usize, y: usize) {
        if self.change_difficulty {
            return;
        }
        if let Some((x, y)) = self.translate_click_coordinates(x, y) {
            self.board.middle_click(x, y);
        }
    }

    pub const fn difficulty(&self) -> Difficulty {
        self.board.difficulty
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.board.difficulty = difficulty;
        self.new_game();
    }

    pub fn get_board_widget(&self) -> Paragraph {
        let time = self.board.first_move_time().map_or_else(
            || String::from(" - 0s"),
            |start| {
                self.board.last_move_time().map_or_else(
                    || format!(" - {:?}s", start.elapsed().as_secs()),
                    |end| format!(" - {:?}s", end.duration_since(start).as_secs()),
                )
            },
        );
        let title = Line::default().spans(vec![
            Span::raw("Minesweeper: ").bold(),
            self.difficulty().as_span(),
            Span::raw(time),
        ]);
        self.board.to_widget().block(
            Block::new()
                .borders(Borders::ALL)
                .title(title)
                .title_alignment(Alignment::Center),
        )
    }

    pub fn get_leaderboard_widget(&self) -> List {
        let items = self
            .leaderboard
            .iter()
            .filter(|score| score.difficulty() == self.difficulty())
            .map(|score| score.as_list_item());
        List::new(items).block(
            Block::default()
                .borders(Borders::NONE)
                .title("Leaderboard")
                .title_alignment(Alignment::Center),
        )
    }

    pub fn get_board_size_with_border(&self) -> (u16, u16) {
        let (x, y) = self.board.get_board_size();
        #[allow(clippy::cast_possible_truncation)]
        (x as u16 * 3 + 2, y as u16 * 3 + 2)
    }

    pub fn new_game(&mut self) {
        self.leaderboard_updated = false;
        self.board = Board::new(self.board.difficulty);
    }
}
