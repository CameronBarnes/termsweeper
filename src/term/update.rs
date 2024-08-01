use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

use super::app::App;
use crate::types::Difficulty;

pub fn handle_keys(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q' | 'Q') => {
            if app.change_difficulty {
                app.change_difficulty = false;
            } else {
                app.should_quit = true;
            }
        }
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.should_quit = true;
            } else {
                app.change_difficulty = !app.change_difficulty;
            }
        }
        KeyCode::Enter => app.change_difficulty = false,
        KeyCode::Char('r' | 'R') => app.new_game(),
        KeyCode::Up => {
            if app.change_difficulty {
                match app.difficulty() {
                    Difficulty::Easy => app.set_difficulty(Difficulty::Hard),
                    Difficulty::Medium => app.set_difficulty(Difficulty::Easy),
                    Difficulty::Hard => app.set_difficulty(Difficulty::Medium),
                }
            }
        }
        KeyCode::Down => {
            if app.change_difficulty {
                match app.difficulty() {
                    Difficulty::Easy => app.set_difficulty(Difficulty::Medium),
                    Difficulty::Medium => app.set_difficulty(Difficulty::Hard),
                    Difficulty::Hard => app.set_difficulty(Difficulty::Easy),
                }
            }
        }
        _ => {}
    }
}

pub fn handle_mouse(app: &mut App, mouse_event: MouseEvent) {
    let x = mouse_event.column;
    let y = mouse_event.row;
    if let MouseEventKind::Up(button) = mouse_event.kind {
        match button {
            MouseButton::Left => app.left_click(x.into(), y.into()),
            MouseButton::Right => app.right_click(x.into(), y.into()),
            MouseButton::Middle => app.middle_click(x.into(), y.into()),
        }
    }
}
