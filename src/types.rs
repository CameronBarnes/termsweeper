use std::{fmt::Display, time::Instant};

use rand::prelude::*;
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Easy => write!(f, "Easy"),
            Self::Medium => write!(f, "Medium"),
            Self::Hard => write!(f, "Hard"),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TileState {
    Hidden,
    Question,
    Marked,
    Visible,
}

#[derive(Clone)]
pub struct Tile {
    is_mine: bool,
    state: TileState,
    bombs_near: usize,
}

impl Tile {
    pub const fn is_mine(&self) -> bool {
        self.is_mine
    }

    pub const fn tile_state(&self) -> TileState {
        self.state
    }

    pub fn set_state(&mut self, state: TileState) {
        self.state = state;
    }

    pub const fn bombs_near(&self) -> usize {
        self.bombs_near
    }

    pub fn as_span(&self, sub_line: usize) -> Vec<Span> {
        match sub_line {
            0 => {
                let span = Span::raw("▗▄▖");
                if self.tile_state() == TileState::Visible {
                    if self.is_mine() {
                        vec![span.red()]
                    } else if self.bombs_near() == 0 {
                        vec![Span::raw("   ")]
                    } else {
                        vec![span.dark_gray()]
                    }
                } else {
                    vec![span]
                }
            }
            1 => match self.tile_state() {
                TileState::Hidden => {
                    vec![Span::raw("▐█▌")]
                }
                TileState::Question => {
                    vec![
                        Span::raw("▐"),
                        Span::raw("?").blue().bold().on_white(),
                        Span::raw("▌"),
                    ]
                }
                TileState::Marked => {
                    vec![
                        Span::raw("▐"),
                        Span::raw("⚑").red().bold().on_white(),
                        Span::raw("▌"),
                    ]
                }
                TileState::Visible => {
                    if self.is_mine() {
                        vec![
                            Span::raw("▐").red(),
                            Span::raw("*").bold().on_red(),
                            Span::raw("▌").red(),
                        ]
                    } else if self.bombs_near() == 0 {
                        vec![Span::raw("   ")]
                    } else {
                        vec![
                            Span::raw("▐").dark_gray(),
                            Span::raw(self.bombs_near().to_string())
                                .bold()
                                .on_dark_gray(),
                            Span::raw("▌").dark_gray(),
                        ]
                    }
                }
            },
            2 => {
                let span = Span::raw("▝▀▘");
                if self.tile_state() == TileState::Visible {
                    if self.is_mine() {
                        vec![span.red()]
                    } else if self.bombs_near() == 0 {
                        vec![Span::raw("   ")]
                    } else {
                        vec![span.dark_gray()]
                    }
                } else {
                    vec![span]
                }
            }
            _ => panic!(),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            is_mine: false,
            state: TileState::Hidden,
            bombs_near: 0,
        }
    }
}

pub fn do_around(
    x: usize,
    y: usize,
    tiles: &mut [Vec<Tile>],
    func: fn(&mut Tile) -> bool,
) -> Vec<(usize, usize)> {
    let mut matching: Vec<(usize, usize)> = Vec::new();
    // above
    if let Some(tile) = tiles
        .get_mut(x)
        .and_then(|x| x.get_mut(y.overflowing_sub(1).0))
    {
        if func(tile) {
            matching.push((x, y - 1));
        }
    }
    // bellow
    if let Some(tile) = tiles.get_mut(x).and_then(|x| x.get_mut(y + 1)) {
        if func(tile) {
            matching.push((x, y + 1));
        }
    }
    // left
    if let Some(tile) = tiles
        .get_mut(x.overflowing_sub(1).0)
        .and_then(|x| x.get_mut(y))
    {
        if func(tile) {
            matching.push((x - 1, y));
        }
    }
    // right
    if let Some(tile) = tiles.get_mut(x + 1).and_then(|x| x.get_mut(y)) {
        if func(tile) {
            matching.push((x + 1, y));
        }
    }
    // above right
    if let Some(tile) = tiles.get_mut(x + 1).and_then(|x| x.get_mut(y + 1)) {
        if func(tile) {
            matching.push((x + 1, y + 1));
        }
    }
    // above left
    if let Some(tile) = tiles
        .get_mut(x.overflowing_sub(1).0)
        .and_then(|x| x.get_mut(y + 1))
    {
        if func(tile) {
            matching.push((x - 1, y + 1));
        }
    }
    // bellow right
    if let Some(tile) = tiles
        .get_mut(x + 1)
        .and_then(|x| x.get_mut(y.overflowing_sub(1).0))
    {
        if func(tile) {
            matching.push((x + 1, y - 1));
        }
    }
    // bellow left
    if let Some(tile) = tiles
        .get_mut(x.overflowing_sub(1).0)
        .and_then(|x| x.get_mut(y.overflowing_sub(1).0))
    {
        if func(tile) {
            matching.push((x - 1, y - 1));
        }
    }
    matching
}

fn gen_tiles(difficulty: Difficulty) -> Vec<Vec<Tile>> {
    let (max_x, max_y, mut mines) = match difficulty {
        Difficulty::Easy => (10, 8, 10),
        Difficulty::Medium => (18, 14, 40),
        Difficulty::Hard => (40, 12, 99),
    };
    let mut tiles = vec![vec![Tile::default(); max_y]; max_x];

    let mut rng = thread_rng();
    while mines > 0 {
        let x = rng.gen_range(0..max_x);
        let y = rng.gen_range(0..max_y);
        if tiles[x][y].is_mine {
            continue;
        }
        tiles[x][y].is_mine = true;
        // Handle numbers for all tiles near
        do_around(x, y, &mut tiles, |tile| {
            tile.bombs_near += 1;
            false
        });
        mines -= 1;
    }
    tiles
}

pub struct Board {
    pub difficulty: Difficulty,
    tiles: Vec<Vec<Tile>>,
    game_over: Option<Instant>,
    first_move: Option<Instant>,
}

impl Board {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            tiles: gen_tiles(difficulty),
            game_over: None,
            first_move: None,
        }
    }

    pub const fn first_move_time(&self) -> Option<Instant> {
        self.first_move
    }

    pub const fn last_move_time(&self) -> Option<Instant> {
        self.game_over
    }

    pub fn to_widget(&self) -> Paragraph {
        let max_y = self.tiles[0].len();
        let max_x = self.tiles.len();

        let mut text = Vec::with_capacity(max_y);
        for y in 0..max_y {
            for sub_line in 0..3 {
                let mut span_vec = Vec::with_capacity(max_x);
                for x in 0..max_x {
                    span_vec.append(&mut self.tiles[x][y].as_span(sub_line));
                }
                text.push(Line::from(span_vec));
            }
        }
        Paragraph::new(text)
    }

    pub fn left_click(&mut self, x: usize, y: usize) {
        if self.game_over.is_some() {
            return;
        }
        let mut tile = self.tiles.get_mut(x).and_then(|x| x.get_mut(y));

        while self.first_move.is_none()
            && tile
                .as_ref()
                .map_or(false, |tile| tile.is_mine() || tile.bombs_near() > 0)
        {
            self.tiles = gen_tiles(self.difficulty);
            tile = self.tiles.get_mut(x).and_then(|x| x.get_mut(y));
        }

        if self.first_move.is_none() {
            self.first_move = Some(Instant::now());
        }

        let mut flood = false;
        if let Some(tile) = tile {
            if tile.tile_state() == TileState::Marked {
                return;
            } else if tile.is_mine() {
                tile.set_state(TileState::Visible);
                self.game_over = Some(Instant::now());
            } else if tile.state != TileState::Visible {
                tile.set_state(TileState::Visible);
                flood = tile.bombs_near() == 0;
            }
        }
        if flood {
            self.flood_fill(x, y);
        }
        if self.game_over.is_some() {
            for tile in self.tiles.iter_mut().flat_map(|vec| vec.iter_mut()) {
                if tile.is_mine() {
                    tile.set_state(TileState::Visible);
                }
            }
        }
    }

    pub fn do_control_click(&mut self, x: usize, y: usize) {
        if self.game_over.is_some() {
            return;
        }
        let mut tiles_to_left_click = Vec::new();
        let tile = self.tiles.get(x).and_then(|x| x.get(y));
        if tile.is_some() {
            let num_around = tile.unwrap().bombs_near();
            let marked_around = do_around(x, y, &mut self.tiles, |tile| {
                tile.tile_state() == TileState::Marked
            })
            .len();
            if num_around == marked_around {
                tiles_to_left_click.append(&mut do_around(x, y, &mut self.tiles, |tile| {
                    tile.tile_state() != TileState::Marked
                        && tile.tile_state() != TileState::Visible
                }));
            }
        }
        for (x, y) in tiles_to_left_click {
            self.left_click(x, y);
            self.do_control_click(x, y);
        }
    }

    pub fn right_click(&mut self, x: usize, y: usize) {
        if self.game_over.is_some() {
            return;
        }
        let tile = self.tiles.get_mut(x).and_then(|x| x.get_mut(y));
        if let Some(tile) = tile {
            if self.first_move.is_none() {
                self.first_move = Some(Instant::now());
            }
            if tile.tile_state() == TileState::Visible {
            } else if tile.tile_state() == TileState::Marked {
                tile.set_state(TileState::Hidden);
            } else {
                tile.set_state(TileState::Marked);
            }
        }
        if self
            .tiles
            .iter()
            .flat_map(|vec| vec.iter())
            .filter(|tile| tile.is_mine())
            .all(|tile| tile.tile_state() == TileState::Marked)
        {
            self.game_over = Some(Instant::now());
            for tile in self.tiles.iter_mut().flat_map(|vec| vec.iter_mut()) {
                if !tile.is_mine() {
                    tile.set_state(TileState::Visible);
                }
            }
        }
    }

    pub fn middle_click(&mut self, x: usize, y: usize) {
        if self.game_over.is_some() {
            return;
        }
        let tile = self.tiles.get_mut(x).and_then(|x| x.get_mut(y));
        if let Some(tile) = tile {
            if self.first_move.is_none() {
                self.first_move = Some(Instant::now());
            }
            if tile.tile_state() == TileState::Visible {
            } else if tile.tile_state() == TileState::Question {
                tile.set_state(TileState::Hidden);
            } else {
                tile.set_state(TileState::Question);
            }
        }
    }

    pub fn flood_fill(&mut self, x: usize, y: usize) {
        let mut around = vec![(x, y)];
        while let Some((x, y)) = around.pop() {
            around.append(&mut do_around(x, y, &mut self.tiles, |tile| {
                if tile.state == TileState::Visible {
                    false
                } else if tile.bombs_near() == 0 && !tile.is_mine() {
                    tile.set_state(TileState::Visible);
                    true
                } else {
                    tile.set_state(TileState::Visible);
                    false
                }
            }));
        }
    }

    pub fn get_board_size(&self) -> (usize, usize) {
        let x = self.tiles.len();
        let y = self.tiles[0].len();
        (x, y)
    }
}
