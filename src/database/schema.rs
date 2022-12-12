// @generated automatically by Diesel CLI.

diesel::table! {
    msgs (id) {
        id -> Int4,
        content -> Text,
        date_time -> Timestamp,
        likes -> Int8,
    }
}
