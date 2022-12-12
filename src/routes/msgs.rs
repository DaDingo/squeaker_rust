use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;

use crate::database::connection::DbPool;
use crate::database::models;
use crate::database::schema;
use crate::error::MyError;

use models::*;

#[get("/msgs")]
async fn get_messages(pool: web::Data<DbPool>) -> Result<HttpResponse, MyError> {
    use self::schema::msgs::dsl::*;

    let mut conn = pool.get()?;

    let results = msgs.load::<Message>(&mut conn)?;
    Ok(HttpResponse::Ok().json(results))
}
