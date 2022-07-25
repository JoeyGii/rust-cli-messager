// use crate::audio_handlers;
use crate::db;
use crate::error_handler::CustomError;
use crate::schema::wiggles_user::dsl::*;
use crate::schema::{message, wiggles_user};
use diesel::prelude::*;
use rand::Rng;
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
    pub fn message_declaration() -> Message {
        let new_message = Message {
            id: 12345,
            name: String::from("new_message"),
            body: String::from("new_message body"),
            published: false,
        };
        new_message
    }
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

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Queryable)]
#[table_name = "wiggles_user"]
pub struct WigglesUser {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
}
impl WigglesUser {
    pub fn get() -> Result<Vec<WigglesUser>, CustomError> {
        let conn = db::connection();
        let get_users = wiggles_user::table.load::<WigglesUser>(&conn)?;
        Ok(get_users)
    }

    pub fn get_by_email(user_email: String) -> Result<Vec<WigglesUser>, CustomError> {
        let conn = db::connection();
        let results = wiggles_user
            .filter(email.eq(user_email))
            .limit(5)
            .load::<WigglesUser>(&conn)
            .expect("Email does not match User");
        Ok(results)
    }
}
impl Default for WigglesUser {
    fn default() -> WigglesUser {
        let mut rng = rand::thread_rng();
        WigglesUser {
            id: rng.gen(),
            name: String::new(),
            password: String::new(),
            email: String::new(),
        }
    }
}
