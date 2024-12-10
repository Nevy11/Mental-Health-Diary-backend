use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::{QuestionDelete, QuestionReturning},
    schema::{questions, questions::dsl::question},
};

pub fn delete_question(data: QuestionDelete) -> Result<QuestionReturning, diesel::result::Error> {
    let connection = &mut establish_connection();

    diesel::delete(questions::dsl::questions)
        .filter(question.eq(data.question.to_uppercase()))
        .returning(QuestionReturning::as_returning())
        .get_result(connection)
}
