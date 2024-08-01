use ratatui::{
    prelude::{Alignment, Constraint::*, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::types::Difficulty;

use super::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let (min_x, min_y) = app.get_board_size_with_border();
    let spare_x = f.size().width.saturating_sub(min_x);
    let spare_y = f.size().height.saturating_sub(min_y);
    let vertical = Layout::new(
        Direction::Vertical,
        [
            Length(spare_y.saturating_div(2)),
            Min(min_y),
            Length(spare_y.saturating_div(2)),
        ],
    )
    .split(f.size());
    let horizontal = Layout::new(
        Direction::Horizontal,
        [
            Length(spare_x.saturating_div(2)),
            Min(min_x),
            Length(spare_x.saturating_div(2)),
        ],
    )
    .split(vertical[1]);
    app.board_rect = horizontal[1];

    // Render game board
    f.render_widget(app.get_board_widget().centered(), app.board_rect);

    // Render game controls
    let instructions = Paragraph::new(
        r"
Left click to uncover tiles

Double click to uncover tiles near marked mines

Middle click to mark a tile as '?'

Right click to flag a tile as a mine

Press R to restart

Press C to change the difficulty",
    )
    .centered()
    .wrap(Wrap { trim: true });
    f.render_widget(instructions, horizontal[0]);

    // Render leaderboard
    f.render_widget(app.get_leaderboard_widget(), horizontal[2]);

    // Render difficulty change ui if requested
    if app.change_difficulty {
        let size_x = 30;
        let size_y = 8;
        let spare_x = f.size().width.saturating_sub(size_x);
        let spare_y = f.size().height.saturating_sub(size_y);
        let vertical = Layout::new(
            Direction::Vertical,
            [
                Length(spare_y.saturating_div(2)),
                Min(size_y),
                Length(spare_y.saturating_div(2)),
            ],
        )
        .split(f.size());
        let horizontal = Layout::new(
            Direction::Horizontal,
            [
                Length(spare_x.saturating_div(2)),
                Min(size_x),
                Length(spare_x.saturating_div(2)),
            ],
        )
        .split(vertical[1]);
        let items = vec![Difficulty::Easy, Difficulty::Medium, Difficulty::Hard]
            .into_iter()
            .map(|difficulty| ListItem::from(difficulty.as_span()));
        let index = match app.difficulty() {
            Difficulty::Easy => 0,
            Difficulty::Medium => 1,
            Difficulty::Hard => 2,
        };
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Difficulty")
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(Style::new().reversed());
        f.render_widget(Clear, horizontal[1]);
        f.render_stateful_widget(
            list,
            horizontal[1],
            &mut ListState::default().with_selected(Some(index)),
        );
    }
}
