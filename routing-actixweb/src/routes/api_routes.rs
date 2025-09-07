use actix_web::web;

use crate::controllers::home_controller;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/index", web::get().to(home_controller::index))
            .route("/user", web::get().to(home_controller::user)),
    );
}
