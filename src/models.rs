use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QuestionAnswer {
    pub username: String,
    pub question: String,
    pub answer: String,
}
