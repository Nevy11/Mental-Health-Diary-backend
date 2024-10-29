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
#[derive(Insertable, Selectable, Queryable, Deserialize, Debug, Serialize)]
#[diesel(table_name = crate::schema::goals_db)]
#[diesel(check_for_backend(Pg))]
pub struct Goals {
    pub id: i32,
    pub username: String,
    pub goal_name: String,
}

#[derive(Deserialize)]
pub struct SearchGoal {
    pub username: String,
}

#[derive(Insertable, Deserialize, Debug, Serialize)]
#[diesel(table_name = crate::schema::goals_db)]
#[diesel(check_for_backend(Pg))]
pub struct Goal {
    pub username: String,
    pub goal_name: String,
}

#[derive(Deserialize)]
pub struct UpdateGoal {
    pub username: String,
    pub old_value: String,
    pub new_value: String,
}

#[derive(Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Insertable, Selectable, Queryable, Deserialize, Debug, Serialize)]
#[diesel(table_name = crate::schema::goals_done)]
#[diesel(check_for_backend(Pg))]
pub struct GoalsDone {
    pub id: i32,
    pub username: String,
    pub goal_name: String,
}

#[derive(Insertable, Deserialize, Debug, Serialize)]
#[diesel(table_name = crate::schema::goals_done)]
#[diesel(check_for_backend(Pg))]
pub struct GoalDone {
    pub username: String,
    pub goal_name: String,
}

#[derive(Serialize)]
pub struct ErrorReturn {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct SuccessReturn {
    pub success: bool,
    pub username: String,
    pub goal_name: String,
}

#[derive(Serialize)]
pub struct SuccessReadOne {
    pub success: bool,
    pub data: Vec<GoalsDone>,
}
