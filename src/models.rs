use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QuestionAnswer {
    pub username: String,
    pub question: String,
    pub answer: String,
}

#[derive(Selectable, Queryable, Insertable, Deserialize)]
#[diesel(table_name=crate::schema::chat_users)]
#[diesel(check_for_backend(Pg))]
pub struct SignUp {
    pub username: String,
    pub userpassword: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Insertable, Selectable, Queryable, Deserialize)]
#[diesel(table_name = crate::schema::chat_users)]
#[diesel(check_for_backend(Pg))]
pub struct LoginChatUsers {
    pub username: String,
    pub userpassword: String,
}

#[derive(Deserialize)]
pub struct UpdateUserPassword {
    pub username: String,
    pub current_password: String,
    pub new_password: String,
}

#[derive(Insertable, Selectable, Queryable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::chat_users)]
#[diesel(check_for_backend(Pg))]
pub struct ChatUsers {
    pub username: String,
    pub userpassword: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateUsernameOrEmail {
    pub username: String,
    pub field: String,
    pub new_value: String,
}

#[derive(Serialize)]
pub struct IsSuccessful {
    pub is_it: bool,
}

#[derive(Deserialize)]
pub struct DeleteUserPassword {
    pub username: String,
    pub current_password: String,
}
#[derive(Insertable, Selectable, Queryable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::goals_db)]
#[diesel(check_for_backend(Pg))]
pub struct Goals {
    pub username: String,
    pub goal_name: String,
}
