use actix_settings::BasicSettings;
use actix_web::{get, web, HttpResponse, Responder};

use crate::settings::Opts;

#[get("/health")]
async fn health(settings: web::Data<BasicSettings<Opts>>) -> impl Responder {
    HttpResponse::Ok().body(settings.application.health_body.to_owned())
}
