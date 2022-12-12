use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub date_time: SystemTime,
    pub likes: i64,
}

use super::schema::msgs;
#[derive(Insertable)]
#[diesel(table_name = msgs)]
pub struct NewMessage<'a> {
    pub content: &'a str,
    pub date_time: &'a SystemTime,
    pub likes: i64,
}
