use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, ListState, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Finish this application!"),
    });
    state.items.push(TodoItem {
        is_done: true,
        description: String::from("Go to gym!"),
    });
    state.items.push(TodoItem {
        is_done: false,
        description: String::from("Fix all the bugs!"),
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
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char(char) => match char {
                    'D' => {
                        if let Some(index) = app_state.list_state.selected() {
                            app_state.items.remove(index);
                        };
                    }
                    'k' => app_state.list_state.select_previous(),
                    'j' => app_state.list_state.select_next(),
                    _ => {}
                },
                _ => {}
            }
        };
    }

    Ok(())
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
            .map(|x| ListItem::from(x.description.clone())),
    )
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Green));
    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);

    // Paragraph::new("Hello from Ratatui Todo App!").render(frame.area(), frame.buffer_mut());
}
