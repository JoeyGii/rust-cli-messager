use crate::app_inputs::{App, InputMode};
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
                Span::styled("n", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to edit your name. This is wiggle walkie talkie. 📟"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Name => (
            vec![
                Span::raw("  Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to remain anonymous, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record your name 📟"),
            ],
            Style::default(),
        ),
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
            InputMode::Name => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input")
                .style(Style::default().fg(Color::LightBlue)),
        );
    f.render_widget(input, chunks[1]);
    let copyright = render_copyright();
    f.render_widget(copyright, chunks[3]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Name => {
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

    //TO DO Check the syntax below. Look up .enumerate and .map
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

    f.render_widget(messages, chunks[2]);
}
