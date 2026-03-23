use crate::types::AppState;

use ratatui::crossterm::event::{self, KeyEvent};
use std::time::Instant;

pub fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Enter => {
            return true;
        }
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Char(char) => match char {
            'A' => {
                app_state.is_add_new = true;
            }
            'D' => {
                if let Some(index) = app_state.list_state.selected() {
                    if app_state.items[index].is_done {
                        app_state.items.remove(index);
                        app_state.del_count = 0;
                    } else {
                        app_state.del_count += 1;

                        let remaining = 3 - app_state.del_count;

                        if app_state.del_count >= 3 {
                            app_state.items.remove(index);
                            app_state.del_count = 0;
                            app_state.error_message = Some(format!("Task [DELETED] Successfully!"));
                        } else {
                            app_state.error_message = Some(format!(
                                "[WARNING]: Complete task first! Press D {} more time(s) to force delete.",
                                remaining
                            ));
                            app_state.error_time = Some(Instant::now());
                        }
                    }
                }
            }
            'j' => match app_state.list_state.selected() {
                None => app_state.list_state.select(Some(0)),
                Some(current) => {
                    app_state.del_count = 0;
                    if current < app_state.items.len().saturating_sub(1) {
                        app_state.list_state.select(Some(current + 1));
                    }
                }
            },
            'k' => match app_state.list_state.selected() {
                None => app_state.list_state.select(Some(0)),
                Some(current) => {
                    app_state.del_count = 0;
                    if current > 0 {
                        app_state.list_state.select(Some(current - 1));
                    }
                }
            },
            _ => {}
        },
        _ => {}
    }
    false
}
