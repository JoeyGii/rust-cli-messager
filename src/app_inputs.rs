use crate::events::{consumer, producer};
use crate::model::models::{Message, WigglesUser};
use crate::ui_render_handler;
use crossbeam_channel::TryRecvError::{self};
use crossbeam_channel::{unbounded, Receiver, Sender};
use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use std::{error::Error, thread};
use tui::{backend::Backend, Terminal};
pub enum InputMode {
    Normal,
    Editing,
    Login,
}
pub enum LoginInput {
    UserName,
    Password,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<Message>,
    pub login_input_mode: LoginInput,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            login_input_mode: LoginInput::UserName,
        }
    }
}
pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    mut user: WigglesUser,
) -> Result<(), Box<dyn Error>> {
    app.messages = ui_render_handler::remove_old_messages(Message::get().unwrap());
    let (sender, receiver): (Sender<String>, Receiver<String>) = unbounded();
    thread::Builder::new()
        .name("kafka consumer thread".to_string())
        .spawn(move || consumer::start_consuming(sender.clone()).unwrap())
        .unwrap();

    loop {
        //event consumer
        app.messages = ui_render_handler::remove_old_messages(app.messages);
        terminal.draw(|f| ui_render_handler::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('l') => {
                        app.input_mode = InputMode::Login;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Login => match key.code {
                    KeyCode::Enter => match app.login_input_mode {
                        LoginInput::UserName => {
                            user.name = app.input.drain(..).collect();
                            app.login_input_mode = LoginInput::Password;
                        }
                        LoginInput::Password => {
                            user.password = app.input.drain(..).collect();
                            app.input_mode = InputMode::Editing;
                        }
                    },
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

                        let mut rng = rand::thread_rng();
                        let message = Message {
                            id: rng.gen(),
                            name: user.name.to_string(),
                            body: body,
                            published: true,
                        };

                        if message.body != "" {
                            let new_body = serde_json::to_string(&message).unwrap();

                            app.messages.push(message.clone());
                            thread::spawn(move || {
                                message.insert().unwrap();
                            });
                            thread::spawn(move || {
                                producer::produce_event(new_body).unwrap();
                            });
                            thread::spawn(move || {});
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
        let message_receiver: Result<String, TryRecvError> = receiver.try_recv();
        match message_receiver {
            Ok(message_receiver) => app
                .messages
                .push(serde_json::from_str(&message_receiver).unwrap()),
            _ => (),
        };
    }
}
