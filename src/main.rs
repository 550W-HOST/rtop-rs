use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rtop_rust::app::{App, AppResult};
use rtop_rust::event::{Event, EventHandler};
use rtop_rust::handler::handle_key_events;
use rtop_rust::tui::Tui;
use std::io;

use heim::cpu;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    let freq = cpu::frequency()
        .await
        .unwrap()
        .current()
        .get::<heim::units::frequency::megahertz>();
    println!("CPU frequency: {} MHz", freq);

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
