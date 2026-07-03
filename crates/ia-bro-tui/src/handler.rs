//! Event handler for IA,BRO! TUI

use crate::app::{App, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key)?,
        InputMode::Creating | InputMode::Editing => handle_input_mode(app, key)?,
    }
    Ok(())
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        // Navigation
        KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
        KeyCode::Down | KeyCode::Char('j') => app.select_next(),

        // Actions
        KeyCode::Char('n') => app.input_mode = InputMode::Creating,
        KeyCode::Char('d') => app.delete_selected()?,
        KeyCode::Enter => app.toggle_selected()?,
        KeyCode::Char('f') => app.cycle_filter(),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.clear_all()?,
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,

        _ => {}
    }
    Ok(())
}

fn handle_input_mode(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.input_buffer.clear();
        }
        KeyCode::Enter => {
            if !app.input_buffer.is_empty() {
                app.add_task(app.input_buffer.clone())?;
                app.input_buffer.clear();
            }
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        KeyCode::Char(c) => app.input_buffer.push(c),
        _ => {}
    }
    Ok(())
}
