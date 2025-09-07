use actix_web::{App, HttpServer};

mod controllers;
mod model;
mod routes;

use routes::api_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(api_routes::api_routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
