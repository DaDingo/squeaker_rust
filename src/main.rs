#[macro_use]
extern crate log;

use std::collections::HashMap;
use actix_web::{App, delete, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::middleware::Logger;
use actix_web_prom::PrometheusMetricsBuilder;
use chrono;
use env_logger::Env;
use gethostname::gethostname;


#[post("/msg")]
async fn new_message(body: String) -> impl Responder {
    let _current_date_time = chrono::offset::Local::now();
    HttpResponse::Ok().body(format!("Got message {} at {}", body, _current_date_time))
}

#[get("/msg/{id}")]
async fn get_message(info: web::Path<u64>) -> impl Responder {
    let id = info.into_inner();
    HttpResponse::Ok().body(format!("You requested the msg with id {}. One day soon you will get it. But not today.", id))
}

#[delete("/msg/{id}")]
async fn del_message(info: web::Path<u64>) -> impl Responder {
    let id = info.into_inner();
    HttpResponse::Ok().body(format!("deleted msg with id {}", id))
}

#[post("/like/{id}")]
async fn like_msg(info: web::Path<u64>) -> impl Responder {
    let id = info.into_inner();
    HttpResponse::Ok().body(format!("liked msg with id {}", id))
}

#[get("/msgs")]
async fn get_messages() -> impl Responder {
    HttpResponse::Ok().body(format!("You requested all msgs. One day soon you will get it. But not today."))
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn get_hostname() -> String {
    gethostname().to_str().unwrap_or("UNKNOWN").to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut labels = HashMap::new();
    labels.insert("hostname".to_string(), get_hostname());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();


    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Starting up..");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(prometheus.clone())
            .service(web::resource("/health").to(health))
            .service(new_message)
            .service(del_message)
            .service(get_message)
            .service(get_messages)
            .service(like_msg)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

