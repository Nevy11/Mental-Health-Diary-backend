use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::AiReturn,
    schema::ai::{self, answer, question, username},
};

/// This function updates the username of the person updating it based on the new
/// value.
pub fn update_username_ai(
    name_of_user: String,
    new_value: String,
) -> Result<AiReturn, diesel::result::Error> {
    let new_value = new_value.to_uppercase();
    let name_of_user = name_of_user.to_uppercase();
    let connection = &mut establish_connection();
    diesel::update(ai::dsl::ai)
        .filter(username.eq(name_of_user))
        .set(username.eq(new_value))
        .returning(AiReturn::as_returning())
        .get_result(connection)
}

/// This function takes in the name of the user, updates it to uppercase, filter's
/// it's question values, then filters the question to be updated by taking the old
/// value and matching it.
pub fn update_question_ai(
    name_of_user: String,
    old_value: String,
    new_value: String,
) -> Result<AiReturn, diesel::result::Error> {
    let connection = &mut establish_connection();
    let name_of_user = name_of_user.to_uppercase();
    diesel::update(ai::dsl::ai)
        .filter(username.eq(name_of_user))
        .filter(question.eq(old_value))
        .set(question.eq(new_value))
        .returning(AiReturn::as_returning())
        .get_result(connection)
}

/// This function updates the answer in the database.
/// Takes in the name of the updated user then filters the actual question
pub fn update_answer_ai(
    name_of_user: String,
    old_value: String,
    new_value: String,
) -> Result<AiReturn, diesel::result::Error> {
    let connection = &mut establish_connection();
    let name_of_user = name_of_user.to_uppercase();
    diesel::update(ai::dsl::ai)
        .filter(username.eq(name_of_user))
        .filter(answer.eq(old_value))
        .set(answer.eq(new_value))
        .returning(AiReturn::as_returning())
        .get_result(connection)
}
