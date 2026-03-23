use crate::handle_add_new::handle_add_new;
use crate::handle_key::handle_key;
use crate::render::render;
use crate::types::AppState;

use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyEventKind},
};

pub fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;

        //Input handling
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if app_state.is_add_new {
                    if handle_add_new(key, app_state) {
                        app_state.is_add_new = false;
                    }
                } else {
                    if handle_key(key, app_state) {
                        break;
                    }
                }
            }
        };
    }

    Ok(())
}
