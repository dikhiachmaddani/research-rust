use crate::route::user_routes::user_routes;
use actix_web::web;

pub fn api_routes() -> actix_web::Scope {
    web::scope("/api/v1").service(user_routes())
}
