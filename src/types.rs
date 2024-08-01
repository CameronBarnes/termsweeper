use core::panic;
use std::time::{Duration, Instant};

use rand::prelude::*;
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::{ListItem, Paragraph},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub const fn as_static_str(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
        }
    }

    pub fn as_span(self) -> Span<'static> {
        let span = Span::raw(self.as_static_str()).italic();
        match self {
            Self::Easy => span.green(),
            Self::Medium => span.yellow(),
            Self::Hard => span.red(),
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
    fire: bool,
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
        let mut line = match sub_line {
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
                            num_as_span(self.bombs_near()).bold().on_dark_gray(),
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
        };
        if self.fire {
            line = line
                .into_iter()
                .enumerate()
                .map(|(index, span)| if index == 1 { span } else { span.on_red() })
                .collect();
        }
        line
    }
}

fn num_as_span(num: usize) -> Span<'static> {
    assert!(num < 9);
    match num {
        1 => Span::raw("1").light_blue(),
        2 => Span::raw("2").light_green(),
        3 => Span::raw("3").light_red(),
        4 => Span::raw("4").light_magenta(),
        5 => Span::raw("5").light_yellow(),
        6 => Span::raw("6").light_cyan(),
        7 => Span::raw("7").black(),
        8 => Span::raw("8").gray(),
        _ => unreachable!(),
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            is_mine: false,
            state: TileState::Hidden,
            bombs_near: 0,
            fire: false,
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

#[allow(clippy::comparison_chain)]
#[allow(clippy::cast_possible_wrap)]
fn circle_points(center: (usize, usize), x: isize, y: isize) -> Vec<(isize, isize)> {
    let (cx, cy) = (center.0 as isize, center.1 as isize);
    let mut points = Vec::new();
    if x == 0 {
        points.push((cx, cy + y));
        points.push((cx, cy - y));
        points.push((cx + y, cy));
        points.push((cx - y, cy));
    }
    if x == y {
        points.push((cx + x, cy + y));
        points.push((cx - x, cy + y));
        points.push((cx + x, cy - y));
        points.push((cx - x, cy - y));
    } else if x < y {
        points.push((cx + x, cy + y));
        points.push((cx - x, cy + y));
        points.push((cx + x, cy - y));
        points.push((cx - x, cy - y));

        points.push((cx + y, cy + x));
        points.push((cx - y, cy + x));
        points.push((cx + y, cy - x));
        points.push((cx - y, cy - x));
    }
    points
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
fn circle_perimeter(center: (usize, usize), radius: f64) -> Vec<(isize, isize)> {
    let mut x = 0;
    let mut y = radius as isize;
    let mut p = radius.mul_add(-4., 5.) / 4.;

    let mut points = circle_points(center, x, y);
    while x < y {
        x += 1;
        if p < 0. {
            p += 2.0f64.mul_add(x as f64, 1.);
        } else {
            y -= 1;
            p += 2.0f64.mul_add(x as f64 - y as f64, 1.);
        }
        points.append(&mut circle_points(center, x, y));
    }

    points
}

fn gen_tiles(difficulty: Difficulty, table_sizes: &[(usize, usize)]) -> Vec<Vec<Tile>> {
    let ((max_x, max_y), mut mines) = match difficulty {
        Difficulty::Easy => (table_sizes[0], 10),   // 80
        Difficulty::Medium => (table_sizes[1], 40), // 252
        Difficulty::Hard => (table_sizes[2], 99),   // 480
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

const fn check_compatible(board_size: (usize, usize), max_render_size: (u16, u16)) -> bool {
    let (x, y) = (board_size.0 * 3 + 12, board_size.1 * 3 + 2);
    let max_width = max_render_size.0 as usize;
    let max_height = max_render_size.1 as usize;

    x <= max_width && y <= max_height
}

fn get_compatible_sizes(max_render_size: (u16, u16)) -> [(usize, usize); 3] {
    let mut sizes = [(0, 0), (0, 0), (0, 0)];

    if check_compatible((10, 8), max_render_size) {
        sizes[0] = (10, 8);
    } else if check_compatible((8, 10), max_render_size) {
        sizes[0] = (8, 10);
    } else {
        panic!("terminal is too small");
    }

    for pair in [(36, 7), (28, 9), (21, 12), (18, 14)].into_iter().rev() {
        if check_compatible(pair, max_render_size) {
            sizes[1] = pair;
            break;
        } else if check_compatible((pair.1, pair.0), max_render_size) {
            sizes[1] = (pair.1, pair.0);
            break;
        }
    }

    for pair in [(8, 60), (10, 48), (12, 40), (15, 32), (16, 30), (20, 24)]
        .into_iter()
        .rev()
    {
        if check_compatible(pair, max_render_size) {
            sizes[2] = pair;
            break;
        } else if check_compatible((pair.1, pair.0), max_render_size) {
            sizes[2] = (pair.1, pair.0);
            break;
        }
    }

    sizes
}

pub struct Board {
    pub difficulty: Difficulty,
    tiles: Vec<Vec<Tile>>,
    game_over: Option<Instant>,
    first_move: Option<Instant>,
    game_over_pos: (usize, usize),
    game_over_state_counter: f64,
    pub max_render_size: (u16, u16),
}

impl Board {
    pub fn new(difficulty: Difficulty, max_render_size: (u16, u16)) -> Self {
        let sizes = get_compatible_sizes(max_render_size);
        Self {
            difficulty,
            tiles: gen_tiles(difficulty, &sizes),
            game_over: None,
            first_move: None,
            game_over_pos: (0, 0),
            game_over_state_counter: 1.,
            max_render_size,
        }
    }

    pub const fn first_move_time(&self) -> Option<Instant> {
        self.first_move
    }

    pub const fn last_move_time(&self) -> Option<Instant> {
        self.game_over
    }

    pub fn set_max_board_size(&mut self, max_render_size: (u16, u16)) {
        self.max_render_size = max_render_size;
        if self.first_move_time().is_none() {
            self.tiles = gen_tiles(self.difficulty, &get_compatible_sizes(self.max_render_size));
        }
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
            self.tiles = gen_tiles(self.difficulty, &get_compatible_sizes(self.max_render_size));
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
            self.game_over_pos = (x, y);
        }
    }

    pub fn clear_fire(&mut self) {
        for tile in self.tiles.iter_mut().flat_map(|vec| vec.iter_mut()) {
            tile.fire = false;
        }
    }

    fn do_game_over_animation_tile(&mut self, x: usize, y: usize) -> bool {
        if let Some(tile) = self.tiles.get_mut(x).and_then(|col| col.get_mut(y)) {
            if tile.is_mine() {
                tile.set_state(TileState::Visible);
            } else {
                tile.fire = true;
            }
            true
        } else {
            false
        }
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn do_game_over_animation(&mut self) {
        assert!(self.game_over.is_some());
        self.clear_fire();

        let mut updated = false;

        let tiles = circle_perimeter(self.game_over_pos, self.game_over_state_counter);
        for (x, y) in tiles {
            if x < 0 || y < 0 {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if self.do_game_over_animation_tile(x, y) {
                updated = true;
            }
        }
        if !updated {
            for tile in self.tiles.iter_mut().flat_map(|vec| vec.iter_mut()) {
                if tile.is_mine() {
                    tile.set_state(TileState::Visible);
                }
            }
        }
        self.game_over_state_counter += 0.25;
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
        if self.check_all_mine_state(TileState::Marked) {
            self.game_over = Some(Instant::now());
            for tile in self.tiles.iter_mut().flat_map(|vec| vec.iter_mut()) {
                if !tile.is_mine() {
                    tile.set_state(TileState::Visible);
                }
            }
        }
    }

    pub fn check_all_mine_state(&self, state: TileState) -> bool {
        self.tiles
            .iter()
            .flat_map(|vec| vec.iter())
            .filter(|tile| tile.is_mine())
            .all(|tile| tile.tile_state() == state)
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Score {
    difficulty: Difficulty,
    time: Duration,
}

impl Score {
    pub const fn new(difficulty: Difficulty, time: Duration) -> Self {
        Self { difficulty, time }
    }

    pub fn as_string(&self) -> String {
        format!(
            "{}: {}",
            self.difficulty.as_static_str(),
            self.time.as_secs()
        )
    }

    pub const fn time(&self) -> Duration {
        self.time
    }

    pub const fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn as_list_item(&self) -> ListItem {
        let difficulty = self.difficulty.as_span();
        let mid = Span::raw(": ");
        let time = Span::raw(self.time().as_secs().to_string()).blue().bold();
        let text = Line::default()
            .spans(vec![difficulty, mid, time, Span::raw("s")])
            .centered();
        ListItem::new(text)
    }
}
