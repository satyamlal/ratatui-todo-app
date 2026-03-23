use crate::types::{AppState, TodoItem};

use ratatui::crossterm::event::{self, KeyEvent};

pub fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Enter => {
            if !app_state.input_value.is_empty() {
                app_state.items.push(TodoItem {
                    is_done: false,
                    description: "> ".to_string() + app_state.input_value.as_str(),
                });
            }
            app_state.input_value.clear();
            return true;
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        event::KeyCode::Esc => {
            app_state.input_value.clear();
            return true;
        }
        _ => {}
    }
    false
}
