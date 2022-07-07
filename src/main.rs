use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use model::{models::Message, route_handler};
use rand::Rng;
mod error_handler;
mod model {
    pub mod models;
    pub mod route_handler;
}
mod ui_render_handler;
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
pub mod audio_handlers;
pub mod db;
pub mod schema;
use std::{error::Error, io, thread};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
#[macro_use]
extern crate diesel;

enum InputMode {
    Normal,
    Editing,
    Name,
}

// TO DO
// RIGHT NOW ITS NOT DELETING OLD MESSAGES SO ITS CUT OFF

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<Message>,
    user_name: Option<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            user_name: None,
        }
    }
}

#[actix_web::main]
async fn inner_runtime() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| actix_web::App::new().service(route_handler::get_messages))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        inner_runtime().unwrap();
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();

    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    app.messages = ui_render_handler::remove_old_messages(Message::get().unwrap());

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('n') => {
                        app.input_mode = InputMode::Name;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Name => match key.code {
                    KeyCode::Enter => {
                        let name: String = app.input.drain(..).collect();
                        app.user_name = Some(name);
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    _ => {}
                },

                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        //Where Message struct is instantiated
                        let body = app.input.drain(..).collect();
                        let name = |n: &Option<String>| -> String {
                            match n {
                                Some(n) => n.to_string(),
                                None => String::from("Anonymous"),
                            }
                        };
                        let mut rng = rand::thread_rng();
                        let message = Message {
                            id: rng.gen(),
                            name: name(&app.user_name),
                            body: body,
                            published: true,
                        };
                        thread::spawn(move || {
                            audio_handlers::new_message_audio();
                        });

                        app.messages.push(message.clone());
                        message.insert().unwrap();
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

//chunk array
// 0 = top text
// 1 = input box
// 2 = messages
// 3 = copyright
fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, Press "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing. Press "),
                Span::styled("n", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to edit your name. This is wiggle walkie talkie."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Name => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to remain anonymous, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record your name"),
            ],
            Style::default(),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
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
    let copyright = ui_render_handler::render_copyright();
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
