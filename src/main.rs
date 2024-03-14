use ratatui::{prelude::CrosstermBackend, Terminal};
use anyhow::{Result, Ok};
use crate::term::{app::App, event::EventHandler, tui::Tui, update::{handle_keys, handle_mouse}};

mod term;
mod types;
mod io;

fn main() -> Result<()> {
    // Init term ui
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let size = terminal.size().unwrap(); // If this fails we might as well exit now
    let events = EventHandler::new(30);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;
 
    // Build app object
    let mut app = App::new(types::Difficulty::Medium, (size.width, size.height));

    // Do main program loop
    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            term::event::Event::Key(key_event) => handle_keys(&mut app, key_event),
            term::event::Event::Mouse(mouse_event) => handle_mouse(&mut app, mouse_event),
            term::event::Event::Tick => app.tick(),
            term::event::Event::Resize(width, height) => app.update_size((width, height)),
            _ => {}
        }
    }

    // Close down the term ui stuff cleanly
    tui.exit()?;

    Ok(())
}
