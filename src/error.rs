use actix_web::http::header;
use actix_web::{error, HttpResponse};
use actix_web::{http::StatusCode, HttpResponseBuilder};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum MyError {
    #[display(fmt = "Error while getting connection from pool: {}", field)]
    DbPoolingError { field: String },

    #[display(fmt = "Error on query: {}", field)]
    DbQueryError { field: String },

    #[display(fmt = "Error on query result: {}", field)]
    DbQueryResultError { field: String },

    #[display(fmt = "Cannot delete non-existing entry with id = {}", field)]
    DeleteNonExistingEntry { field: String },

    #[display(
        fmt = "Deleted multiple entries with same id ({}), which should never hapen. This is a sign for some inconsistency in the databases primary index key.",
        field
    )]
    DeletedMultipleEntries { field: String },

    #[display(fmt = "Database delete operation returned unknown code ({}).", field)]
    DeleteReturnCodeUnknown { field: String },
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header((header::CONTENT_TYPE, "text/plain; charset=utf-8"))
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl From<r2d2::Error> for MyError {
    fn from(err: r2d2::Error) -> Self {
        MyError::DbPoolingError {
            field: err.to_string(),
        }
    }
}

impl From<diesel::result::Error> for MyError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => MyError::DbQueryResultError {
                field: "No query result found.".to_owned(),
            },
            _ => MyError::DbQueryError {
                field: "Database error.".to_owned(),
            },
        }
    }
}
