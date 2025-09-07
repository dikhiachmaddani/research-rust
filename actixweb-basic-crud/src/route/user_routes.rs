use crate::controllers::user_controller;
use actix_web::{web, Scope};

pub fn user_routes() -> Scope {
    web::scope("/user")
        .route("", web::get().to(user_controller::get_all))
        .route("", web::post().to(user_controller::create))
        .route("{id}", web::get().to(user_controller::get_by_id))
        .route("{id}", web::put().to(user_controller::update))
        .route("{id}", web::delete().to(user_controller::delete))
}
