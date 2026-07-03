//! IA,BRO! TODO TUI Application
//!
//! A beautiful, keyboard-driven terminal application for managing tasks and todos.

use anyhow::Result;
use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ia_bro_tui::{
    app::App,
    event::EventHandler,
    handler,
    ui,
};
use ratatui::prelude::*;
use std::io;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "IA,BRO! TODO")]
#[command(about = "A terminal UI for managing tasks and todos", long_about = None)]
struct Args {
    /// Path to store tasks
    #[arg(short, long)]
    storage: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize app
    let mut app = App::new(args.storage)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Event handler
    let mut events = EventHandler::new(Duration::from_millis(250));

    // Main loop
    let result = app_loop(&mut terminal, &mut app, &mut events).await;

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    result
}

async fn app_loop<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    events: &mut EventHandler,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if app.should_quit {
            break;
        }

        if let Some(event) = events.next().await {
            use ia_bro_tui::event::Event::*;
            match event {
                Key(key) => handler::handle_key(app, key)?,
                Tick => {}
                Quit => break,
            }
        }
    }

    Ok(())
}
