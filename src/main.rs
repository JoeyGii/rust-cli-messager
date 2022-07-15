use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::{consumer, producer};
use model::{models::Message, route_handler};
use rand::Rng;
mod error_handler;
mod model {
    pub mod models;
    pub mod route_handler;
}
mod events {
    pub mod consumer;
    pub mod producer;
    pub mod utils;
}
mod ui_render_handler;
use ui_render_handler::{App, InputMode};
pub mod audio_handlers;
pub mod db;
pub mod schema;
use std::{error::Error, io, thread};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

#[macro_use]
extern crate diesel;

#[actix_web::main]
async fn actix_runtime() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| actix_web::App::new().service(route_handler::get_messages))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //web server
    thread::spawn(move || {
        actix_runtime().unwrap();
    });
    //event consumer
    thread::spawn(move || {
        consumer::start_consuming();
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();

    let res = run_app(&mut terminal, app).await;

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

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Box<dyn Error>> {
    app.messages = ui_render_handler::remove_old_messages(Message::get().unwrap());

    loop {
        app.messages = ui_render_handler::remove_old_messages(app.messages);
        terminal.draw(|f| ui_render_handler::ui(f, &app))?;

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
                        if message.body != "" {
                            let new_body = message.body.to_string();

                            app.messages.push(message.clone());
                            thread::spawn(move || {
                                message.insert().unwrap();
                            });
                            thread::spawn(move || {
                                producer::produce_event(new_body).unwrap();
                            });
                        }
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
