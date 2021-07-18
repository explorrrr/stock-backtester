use actix_web::{middleware::Logger, App, HttpServer};

#[macro_use]
extern crate diesel;

mod config;
mod controllers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = config::setting::get_config();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(controllers::health_check::health_check)
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
}
