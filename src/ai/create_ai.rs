use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::{Ai, AiReturn},
    schema::ai,
};

/// This function takes in username, question and answer to be created then inserts
/// it into the ai database.
pub fn create_ai(data: Ai) -> Result<AiReturn, diesel::result::Error> {
    let connection = &mut establish_connection();
    let created_data = Ai {
        username: data.username.to_uppercase(),
        question: data.question,
        answer: data.answer,
    };
    diesel::insert_into(ai::dsl::ai)
        .values(created_data)
        .returning(AiReturn::as_returning())
        .get_result(connection)
}
