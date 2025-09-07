use actix_web::{web::Data, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use route::api_routes;
use std::env;

mod db;

mod controllers;
mod model;
mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::establish_pool(&database_url).expect("Failed to create pool");
    
    println!("🚀 Server starting on http://{}:{}", host, port);
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(api_routes::api_routes())
    })
    .bind(format!("{}:{}", host, port))?.run().await
}
