mod app;
mod events;
mod ui;

use std::{io, time::Duration};

use app::App;
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use events::Events;
use tui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut rcv = Events::start();

    app.init();

    loop {
        if app.should_quit {
            return Ok(());
        }

        terminal.draw(|f| ui::draw(f, &app))?;

        while let Ok(key) = rcv.try_recv() {
            app.take_action(key.code);
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
        app.tick();
    }
}
