// @generated automatically by Diesel CLI.

diesel::table! {
    chat_users (username) {
        username -> Varchar,
        userpassword -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    goals_db (username) {
        username -> Varchar,
        goal_name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chat_users,
    goals_db,
);
