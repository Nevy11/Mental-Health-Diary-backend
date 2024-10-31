// @generated automatically by Diesel CLI.

diesel::table! {
    chat_users (username) {
        username -> Varchar,
        userpassword -> Varchar,
        email -> Varchar,
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
    chat_users,
    favourite_day,
    goals_db,
    goals_done,
);
