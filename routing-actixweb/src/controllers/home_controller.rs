use actix_web::{HttpResponse, Responder};
use serde_json::json;
use crate::model::user;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Hello world!" }))
}

pub async fn user() -> impl Responder {
    let users = vec![
        user::User {
            name: String::from("Item 1"),
            age: 30,
        },
        user::User {
            name: String::from("Item 2"),
            age: 20,
        },
    ];
    HttpResponse::Ok().json(json!({ "message": "User!", "users": users }))
}