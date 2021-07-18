use actix_web::{middleware::Logger, App, HttpServer};

#[macro_use]
extern crate diesel;

mod config;
mod controllers;
mod domain;
mod infrastructure;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = config::setting::get_config();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(controllers::health_check::health_check)

            // insert time series data
            .service(controllers::price_time_series::store_price_time_series_data)
            // execute back test
            .service(controllers::back_test::back_test)
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
}
