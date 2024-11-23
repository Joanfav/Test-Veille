use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}

#[get("/api/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: String::from("Hello from Rust!"),
    })
}

#[post("/api/echo")]
pub async fn echo(message: web::Json<Message>) -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: format!("Echo: {}", message.content),
    })
}
