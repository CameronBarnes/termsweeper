use ratatui::{prelude::CrosstermBackend, Terminal};
use anyhow::{Result, Ok};
use crate::term::{app::App, event::EventHandler, tui::Tui, update::update};

mod term;

fn main() -> Result<()> {
    println!("Hello, world!");

    // Build app object
    let mut app = App::new();

    // Init term ui
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(16);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Do main program loop
    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            term::event::Event::Key(key_event) => update(&mut app, key_event),
            term::event::Event::Mouse(_) => {
                
            }
            _ => {}
        }
    }

    // Close down the term ui stuff cleanly
    tui.exit()?;

    Ok(())
}
