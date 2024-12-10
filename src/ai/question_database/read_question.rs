use crate::{
    connection::establish_connection::establish_connection,
    models::{QuestionDelete, QuestionReturning},
    schema::questions::{self, question},
};
use diesel::prelude::*;

pub fn read_one_question(
    data: QuestionDelete,
) -> Result<Vec<QuestionReturning>, diesel::result::Error> {
    let connection = &mut establish_connection();
    questions::dsl::questions::filter(questions::table, question.eq(data.question.to_uppercase()))
        .select(QuestionReturning::as_returning())
        .get_results(connection)
}

pub fn read_all_questions() -> Result<Vec<QuestionReturning>, diesel::result::Error> {
    let connection = &mut establish_connection();
    questions::dsl::questions::load::<QuestionReturning>(questions::table, connection)
}
