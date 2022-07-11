// use crate::audio_handlers;
use crate::db;
use crate::error_handler::CustomError;
use crate::schema::message;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[table_name = "message"]
pub struct Message {
    pub id: i32,
    pub name: String,
    pub body: String,
    pub published: bool,
}

impl Message {
    pub fn get() -> Result<Vec<Message>, CustomError> {
        let conn = db::connection();
        let get_messages = message::table.load::<Message>(&conn)?;
        Ok(get_messages)
    }

    pub fn insert(&self) -> Result<Message, CustomError> {
        let conn = db::connection();
        let message = diesel::insert_into(message::table)
            .values(self)
            .get_result(&conn)?;
        Ok(message)
    }

    // pub fn incoming_message(&self) -> Result<Message, CustomError> { thread::spawn(move || {
    //     audio_handlers::new_message_audio();
    // });}

    pub fn clone(&self) -> Message {
        let new_message = Message {
            id: self.id,
            name: self.name.to_string(),
            body: self.body.to_string(),
            published: self.published,
        };
        new_message
    }
}
