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

    // if the cli was called with an argument, then use that as the project_dir
    let project_dir = match std::env::args().nth(1) {
        Some(arg) => arg,
        // otherwise the project_dir is the current dir that user called this program
        None => std::env::current_dir()?.to_str().unwrap().to_string(),
    };
    // create app and run it
    let mut app = App::new(project_dir)?;
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

        match app.handle_events()? {
            true => return Ok(()),
            false => continue,
        }
    }
}
