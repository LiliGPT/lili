mod app;
mod components;
mod shortcuts;
mod utils;
mod views;

use std::{error::Error, io, sync::Mutex};

use anyhow::Result;
use app::{App, AppState};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    let project_dir = project_dir.trim_end_matches('/').to_string();
    // create app and run it
    let state = Mutex::new(AppState::new(project_dir).await?);
    let mut app = App::new(state)?;
    let res = run_app(&mut terminal, &mut app).await;

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

fn draw_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    terminal.draw(|f| {
        app.draw(f).unwrap();
    })?;

    Ok(())
}

pub fn redraw_app(state: &mut AppState) {
    let state = Mutex::new(state.clone());
    let mut app = App::new(state).unwrap();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear();
    draw_app(&mut terminal, &mut app).unwrap();
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        draw_app(terminal, app)?;

        match app.handle_events().await? {
            true => return Ok(()),
            false => continue,
        }
    }
}
