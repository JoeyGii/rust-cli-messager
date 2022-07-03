use crate::model::models::Message;
use crate::{error_handler::CustomError, schema::message};
use actix_web::{get, post, web, HttpResponse};

#[get("/user/{name}")]
pub async fn get_messages() -> Result<HttpResponse, CustomError> {
    let user = Message::get()?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/new-message")]
pub async fn new_message(message: web::Json<Message>) -> Result<HttpResponse, CustomError> {
    let message = Message::insert(message.into_inner())?;
    Ok(HttpResponse::Ok().json(message))
}