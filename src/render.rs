use crate::types::AppState;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, List, ListItem, Padding, Paragraph, Widget},
};

pub fn render(frame: &mut Frame, app_state: &mut AppState) {
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
