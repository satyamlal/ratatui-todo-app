use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget},
};
use std::time::Instant;

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
    error_message: Option<String>,
    error_time: Option<Instant>,
    del_count: i32,
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
        .title(Line::from(" 📝 Todo App ").centered())
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    if app_state.is_add_new {
        let [label_area, input_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                .margin(2)
                .areas(frame.area());

        Paragraph::new("Add new item here: ")
            .fg(Color::White)
            .render(label_area, frame.buffer_mut());

        Paragraph::new("> ".to_string() + app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded),
            )
            .render(input_area, frame.buffer_mut());
    } else {
        let list = List::new(
            app_state
                .items
                .iter()
                .map(|x| ListItem::from(x.description.as_str())),
        )
        .highlight_style(Style::default().fg(Color::Green));
        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }

    if let Some(time) = app_state.error_time {
        if time.elapsed().as_secs() >= 2 {
            app_state.error_message = None;
            app_state.error_time = None;
        }
    }

    if let Some(msg) = &app_state.error_message {
        let [_, msg_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(frame.area());

        Paragraph::new(msg.as_str())
            .fg(Color::Red)
            .block(
                Block::bordered()
                    .fg(Color::Red)
                    .border_type(BorderType::Rounded),
            )
            .render(msg_area, frame.buffer_mut())
    }
}
