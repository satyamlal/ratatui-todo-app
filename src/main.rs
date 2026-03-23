mod handle_add_new;
mod handle_key;
mod render;
mod run;
mod types;

use run::run;
use types::{AppState, TodoItem};

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let mut state = AppState::default();

    state.is_add_new = false;

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("> Finish this application!"),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("> Go to gym!"),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("> Fix all the bugs!"),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("> Do the homework!"),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("> RUST everyday!"),
    });
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}
