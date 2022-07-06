use chrono::prelude::*;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
pub fn render_copyright<'a>() -> Paragraph<'a> {
    let get_current_year = || -> String {
        let current_date = chrono::Utc::now();
        let year = current_date.year();
        year.to_string()
    };
    let copyright = Paragraph::new(format!(
        "Wiggle-CLI {} - all rights reserved",
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
