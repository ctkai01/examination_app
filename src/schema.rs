// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        token -> Varchar,
        avatar -> Nullable<Text>,
        full_name -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        dob -> Timestamp,
        created_at -> Timestamp,
    }
}
