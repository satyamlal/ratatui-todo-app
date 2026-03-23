use ratatui::widgets::ListState;
use std::time::Instant;

#[derive(Debug, Default)]
pub struct AppState {
    pub items: Vec<TodoItem>,
    pub list_state: ListState,
    pub is_add_new: bool,
    pub input_value: String,
    pub error_message: Option<String>,
    pub error_time: Option<Instant>,
    pub del_count: i32,
}

#[derive(Debug, Default)]
pub struct TodoItem {
    pub is_done: bool,
    pub description: String,
}
