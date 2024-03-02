use crossterm::event::{KeyEvent, KeyCode};

use super::app::App;



pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q' | 'Q') => app.should_quit = true,
        _ => {}
    }
}
