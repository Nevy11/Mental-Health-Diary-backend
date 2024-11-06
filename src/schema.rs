// @generated automatically by Diesel CLI.

diesel::table! {
    ai (id) {
        id -> Int4,
        username -> Varchar,
        question -> Varchar,
        answer -> Varchar,
    }
}

diesel::table! {
    chat_users (username) {
        username -> Varchar,
        userpassword -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    diary (username) {
        username -> Varchar,
        content -> Text,
    }
}

diesel::table! {
    favourite_day (username) {
        username -> Varchar,
        day_favourite -> Varchar,
    }
}

diesel::table! {
    goals_db (id) {
        id -> Int4,
        username -> Varchar,
        goal_name -> Varchar,
    }
}

diesel::table! {
    goals_done (id) {
        id -> Int4,
        username -> Varchar,
        goal_name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ai,
    chat_users,
    diary,
    favourite_day,
    goals_db,
    goals_done,
);
