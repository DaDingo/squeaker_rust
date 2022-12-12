extern crate config;

use actix_settings::{ActixSettings, ApplySettings, BasicSettings, Mode};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use database::connection::init_db_pool;
use routes::{del_message, get_message, get_messages, health, like_msg, new_message};

mod database;
mod error;
mod routes;
mod settings;
use settings::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = BasicSettings::<Opts>::read_config().unwrap();
    init_logger(&settings.actix);
    let settings = Data::new(settings);
    let app_settings = Data::clone(&settings);

    let pool = Data::new(init_db_pool(&settings.application.database_url));

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            .app_data(Data::clone(&pool))
            .app_data(Data::clone(&app_settings))
            .service(new_message)
            .service(del_message)
            .service(get_message)
            .service(get_messages)
            .service(like_msg)
            .service(health)
    })
    .apply_settings(settings.get_ref())
    .run()
    .await
}

/// Initialize the logging infrastructure
fn init_logger(settings: &ActixSettings) {
    if !settings.enable_log {
        return;
    }
    std::env::set_var(
        "RUST_LOG",
        match settings.mode {
            Mode::Development => "actix_web=debug",
            Mode::Production => "actix_web=info",
        },
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    println!("Server running at {:?}", settings.hosts);
}
