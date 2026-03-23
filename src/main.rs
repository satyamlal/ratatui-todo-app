use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();

    state.is_add_new = false;

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish this application!"),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Go to gym!"),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Fix all the bugs!"),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Do the homework!"),
    });

    state.items.push(TodoItem {
        is_done: false,
        description: String::from("RUST everyday!"),
    });
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
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

fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Enter => {
            if !app_state.input_value.is_empty() {
                app_state.items.push(TodoItem {
                    is_done: false,
                    description: app_state.input_value.clone(),
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

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
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
                    app_state.items.remove(index);
                };
            }
            'j' => match app_state.list_state.selected() {
                None => app_state.list_state.select(Some(0)),
                Some(current) => {
                    if current < app_state.items.len().saturating_sub(1) {
                        app_state.list_state.select(Some(current + 1));
                    }
                }
            },
            'k' => match app_state.list_state.selected() {
                None => app_state.list_state.select(Some(0)),
                Some(current) => {
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

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(3)
        .areas(frame.area());

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list = List::new(
        app_state
            .items
            .iter()
            .map(|x| ListItem::from(x.description.as_str())),
    )
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Green));
    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);

    if app_state.is_add_new {
        Paragraph::new(app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded),
            )
            .render(frame.area(), frame.buffer_mut());
    }
}
