use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QuestionAnswer {
    pub username: String,
    pub question: String,
    pub answer: String,
}

#[derive(Selectable, Queryable, Insertable)]
#[diesel(table_name=crate::schema::chat_users)]
#[diesel(check_for_backend(Pg))]
pub struct SignUp {
    username: String,
    userpassword: String,
    email: String,
}
