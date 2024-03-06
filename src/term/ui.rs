use ratatui::{Frame, prelude::{Direction, Layout, Constraint::*}};

use super::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let (min_x, min_y) = app.get_board_size_with_border();
    let spare_x = f.size().width - min_x;
    let spare_y = f.size().height - min_y;
    let vertical = Layout::new(Direction::Vertical, [Length(spare_y / 2), Min(min_y), Length(spare_y / 2)]).split(f.size());
    let horizontal = Layout::new(Direction::Vertical, [Length(spare_x / 2), Min(min_x), Length(spare_x / 2)]).split(vertical[1]);
    app.board_rect = horizontal[1];
    println!("{:?}", app.board_rect);

    f.render_widget(app.get_board_widget(), app.board_rect);
}
