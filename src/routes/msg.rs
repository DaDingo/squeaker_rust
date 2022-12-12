use actix_web::{delete, get, post, web, HttpResponse};
use diesel::prelude::*;
use std::time::SystemTime;

use crate::database::connection::DbPool;
use crate::database::models;
use crate::database::schema;
use crate::error::MyError;

use models::*;

#[post("/msg")]
async fn new_message(pool: web::Data<DbPool>, body: String) -> Result<HttpResponse, MyError> {
    use schema::msgs;

    let current_date_time = SystemTime::now();
    let new_msg = NewMessage {
        content: &body,
        date_time: &current_date_time,
        likes: 0,
    };

    let mut conn = pool.get()?;

    let inserted_result = diesel::insert_into(msgs::table)
        .values(&new_msg)
        .get_result::<Message>(&mut conn)?;

    Ok(HttpResponse::Ok().json(inserted_result))
}

#[get("/msg/{id}")]
async fn get_message(
    pool: web::Data<DbPool>,
    info: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    use self::schema::msgs::dsl::*;

    let get_id = info.into_inner();

    let mut conn = pool.get()?;

    let results = msgs
        .filter(id.eq(get_id))
        .get_result::<Message>(&mut conn)?;

    Ok(HttpResponse::Ok().json(results))
}

#[delete("/msg/{id}")]
async fn del_message(
    pool: web::Data<DbPool>,
    info: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    use self::schema::msgs::dsl::*;
    let del_id = info.into_inner();

    let mut conn = pool.get()?;

    let deleted_rows = diesel::delete(msgs.filter(id.eq(del_id))).execute(&mut conn)?;

    match deleted_rows {
        x if x == 1 => Ok(HttpResponse::Ok().body(format!("deleted msg with id {}", del_id))),
        x if x == 0 => Err(MyError::DeleteNonExistingEntry {
            field: del_id.to_string(),
        }),
        x if x > 1 => Err(MyError::DeletedMultipleEntries {
            field: del_id.to_string(),
        }),
        _ => Err(MyError::DeleteReturnCodeUnknown {
            field: del_id.to_string(),
        }),
    }
}
