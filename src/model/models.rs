use crate::db;
use crate::error_handler::CustomError;
use crate::schema::message;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[table_name = "message"]
pub struct Message {
    id: i32,
    name: String,
    body: String,
    published: bool,
}

impl Message {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_body(&self) -> &String {
        &self.body
    }

    pub fn get() -> Result<Vec<Message>, CustomError> {
        let conn = db::connection();
        let get_messages = message::table.load::<Message>(&conn)?;
        Ok(get_messages)
    }

    pub fn insert(message: Message) -> Result<Message, CustomError> {
        let conn = db::connection();
        let message = diesel::insert_into(message::table)
            .values(message)
            .get_result(&conn)?;
        Ok(message)
    }
}
