use crate::events::producer;
use crate::model::models::Message;
use crate::ui_render_handler;
use crate::ui_render_handler::{App, InputMode};
use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::{error::Error, thread};
use tui::{backend::Backend, Terminal};

pub async fn run_app<B: Backend>(
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
