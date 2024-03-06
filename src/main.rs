use crossterm::event::{MouseEventKind, MouseButton};
use ratatui::{prelude::CrosstermBackend, Terminal};
use anyhow::{Result, Ok};
use crate::term::{app::App, event::EventHandler, tui::Tui, update::update};

mod term;
mod types;

fn main() -> Result<()> {
    println!("Hello, world!");

    // Build app object
    let mut app = App::new(types::Difficulty::Medium);

    // Init term ui
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(33);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Do main program loop
    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            term::event::Event::Key(key_event) => update(&mut app, key_event),
            term::event::Event::Mouse(e) => {
                let x = e.column;
                let y = e.row;
                if let MouseEventKind::Up(button) = e.kind {
                    match button {
                        MouseButton::Left => app.left_click(x.into(), y.into()),
                        MouseButton::Right => app.right_click(x.into(), y.into()),
                        MouseButton::Middle => app.middle_click(x.into(), y.into()),
                    }
                }
            }
            //term::event::Event::Tick => app.double_click = None,
            _ => {}
        }
    }

    // Close down the term ui stuff cleanly
    tui.exit()?;

    Ok(())
}
