use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use model::route_handler;
mod error_handler;
mod model {
    pub mod models;
    pub mod route_handler;
}
mod events {
    pub mod consumer;
    pub mod mpsc_channel_handler;
    pub mod producer;
    pub mod utils;
}
mod app_inputs;
mod ui_render_handler;
use app_inputs::App;
pub mod audio_handlers;
pub mod db;
pub mod schema;
use std::{error::Error, io, thread};
use tui::{backend::CrosstermBackend, Terminal};

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

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();

    let res = app_inputs::run_app(&mut terminal, app).await;

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
