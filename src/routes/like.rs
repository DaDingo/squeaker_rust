use actix_web::{post, web, HttpResponse};
use diesel::prelude::*;

use crate::database::connection::DbPool;
use crate::database::models;
use crate::database::schema;
use crate::error::MyError;

use models::*;

#[post("/like/{id}")]
async fn like_msg(pool: web::Data<DbPool>, info: web::Path<i32>) -> Result<HttpResponse, MyError> {
    use self::schema::msgs::dsl::*;
    let msg_id = info.into_inner();
    let mut conn = pool.get()?;
    let result = msgs.filter(id.eq(msg_id)).first::<Message>(&mut conn)?;

    let my_likes = result.likes;

    let mut conn = pool.get()?;

    let _res = diesel::update(msgs.find(msg_id))
        .set(likes.eq(my_likes + 1))
        .get_result::<Message>(&mut conn)?;

    Ok(HttpResponse::Ok().body(format!("liked msg with id {}", msg_id)))
}
