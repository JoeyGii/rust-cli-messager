use std::vec;

use crate::app_inputs::{App, InputMode, LoginInput};
use crate::model::models::Message;
use chrono::prelude::*;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

//chunk array
// 0 = top text
// 1 = input box
// 2 = messages
// 3 = copyright
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Percentage(85),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("  📟 Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, Press "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing. Press "),
                Span::styled("l", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to login. This is walkie talkie wiggles. 📟"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Login => match app.login_input_mode {
            LoginInput::UserName => (
                vec![
                    Span::raw("Enter "),
                    Span::styled(
                        "your username ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("when you are finished, press "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("  📟"),
                ],
                Style::default(),
            ),

            LoginInput::Password => (
                vec![
                    Span::raw("Enter "),
                    Span::styled(
                        "your password ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("when you are finished, press "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("  📟"),
                ],
                Style::default(),
            ),
        },

        InputMode::Editing => (
            vec![
                Span::raw("  Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message 📟"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
            InputMode::Login => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input")
                .style(Style::default().fg(Color::LightBlue)),
        );
    f.render_widget(input, chunks[1]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Login => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(_i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", m.name, m.body)))];
            ListItem::new(content)
        })
        .collect();

    let messages = List::new(messages)
        .style(Style::default().fg(Color::LightCyan))
        .block(
            Block::default()
                .style(Style::default().fg(Color::Blue))
                .borders(Borders::ALL)
                .title("Messages"),
        );
    match app.input_mode {
        InputMode::Normal => f.render_widget(render_home(&app), chunks[2]),
        InputMode::Login => f.render_widget(render_home(&app), chunks[2]),
        InputMode::Editing => f.render_widget(messages, chunks[2]),
    }

    let copyright = render_copyright();
    f.render_widget(copyright, chunks[3]);
}

fn render_home<'a>(app: &App) -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Walkie 📟")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Talkie 📟")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("WIGGLES")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Remember to be nice to your friends.",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        match app.input_mode {
            InputMode::Normal => Spans::from(vec![Span::styled(
                "Press l to login.",
                Style::default().fg(Color::White),
            )]),
            InputMode::Login => Spans::from(vec![Span::styled(
                "Enter your login deetz above.",
                Style::default().fg(Color::White),
            )]),
            InputMode::Editing => Spans::from(vec![Span::raw("")]),
        },
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Blue))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}
pub fn render_copyright<'a>() -> Paragraph<'a> {
    let get_current_year = || -> String {
        let current_date = chrono::Utc::now();
        let year = current_date.year();
        year.to_string()
    };
    let copyright = Paragraph::new(format!(
        "📟 Wiggle-CLI {} - all rights reserved 📟",
        get_current_year()
    ))
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightBlue))
            .title("Copyright")
            .border_type(BorderType::Plain),
    );
    copyright
}

pub fn remove_old_messages(mut messages: Vec<Message>) -> Vec<Message> {
    let message_count = messages.len();
    if message_count > 10 {
        messages.drain(0..message_count - 33);
    }
    messages
}

// pub fn users_ui_renderer<'a>() -> Paragraph<'a> {
//     let input = Paragraph::new(String::from("Hello"))
//         .style(Style::default())
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .title("Input")
//                 .style(Style::default().fg(Color::LightBlue)),
//         );
//     input
// }
