use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::QuestionReturning,
    schema::questions::{self, context, question},
};

pub fn update_question(
    old_question: String,
    new_question: String,
) -> Result<QuestionReturning, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::update(questions::dsl::questions)
        .filter(question.eq(old_question.to_uppercase()))
        .set(question.eq(new_question.to_uppercase()))
        .returning(QuestionReturning::as_returning())
        .get_result(connection)
}

pub fn update_context(
    old_context: String,
    new_context: String,
) -> Result<QuestionReturning, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::update(questions::dsl::questions)
        .filter(context.eq(old_context))
        .set(context.eq(new_context))
        .returning(QuestionReturning::as_returning())
        .get_result(connection)
}
