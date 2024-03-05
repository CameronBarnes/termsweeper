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
                let x = e.column.saturating_sub(1);
                let y = e.row.saturating_sub(1);
                if let MouseEventKind::Up(button) = e.kind {
                    match button {
                        MouseButton::Left => {
                            if app.double_click.is_some_and(|pos| pos.0 == x && pos.1 == y) {
                                app.board.do_control_click(x.into(), y.into());
                                //eprintln!("Double Click");
                            } else {
                                app.board.left_click(x.into(), y.into());
                                app.double_click = Some((x, y));
                            }
                        },
                        MouseButton::Right => app.board.right_click(x.into(), y.into()),
                        MouseButton::Middle => app.board.middle_click(x.into(), y.into()),
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
