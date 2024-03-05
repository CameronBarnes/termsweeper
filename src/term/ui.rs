use ratatui::Frame;

use super::app::App;

pub fn render(app: &App, f: &mut Frame) {
    f.render_widget(app.board.to_widget(), f.size());
}
