use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
use chrono;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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