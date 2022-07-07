use crate::error_handler::CustomError;
use crate::model::models::Message;
use actix_web::{get, HttpResponse};

#[get("/user/{name}")]
pub async fn get_messages() -> Result<HttpResponse, CustomError> {
    let user = Message::get()?;
    Ok(HttpResponse::Ok().json(user))
}
