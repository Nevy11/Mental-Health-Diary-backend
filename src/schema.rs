// @generated automatically by Diesel CLI.

diesel::table! {
    chat_users (username) {
        username -> Varchar,
        userpassword -> Varchar,
        email -> Varchar,
    }
}
