mod app;
mod components;
mod utils;

use std::{error::Error, io};

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new()?;
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| {
            app.draw(f).unwrap();
        })?;

        if let Event::Key(key) = event::read()? {
            let context_files = &mut app.el_context_files;
            let message = &mut app.el_message;

            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
            if let KeyCode::Char('c') = key.code {
                message.set_focus(false);
                context_files.set_focus(true);
            }
            if let KeyCode::Char('i') = key.code {
                message.set_focus(true);
                context_files.set_focus(false);
            }
            if let KeyCode::Up = key.code {
                context_files.select_previous();
            }
            if let KeyCode::Down = key.code {
                context_files.select_next();
            }
        }
    }
}
