use crate::{
    connection::establish_connection::establish_connection,
    models::{Question, QuestionReturning},
    schema::questions,
};
use diesel::prelude::*;

pub fn create_question(data: Question) -> Result<QuestionReturning, diesel::result::Error> {
    let input_question = Question {
        question: data.question.clone().to_uppercase(),
        context: data.context.clone(),
    };
    let connection = &mut establish_connection();
    diesel::insert_into(questions::dsl::questions)
        .values(input_question)
        .returning(QuestionReturning::as_returning())
        .get_result(connection)
}
