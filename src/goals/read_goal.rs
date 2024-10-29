use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::{Goals, SearchGoal},
    schema::goals_db::{self, username},
};
/// This function reads all the goals from the database returning
/// all the goals set by the users using the mental diary application.
/// This function is insecure and should be used with at most care.
pub fn read_all_goal() -> Result<Vec<Goals>, diesel::result::Error> {
    let connection = &mut establish_connection();
    goals_db::dsl::goals_db::load::<Goals>(goals_db::table, connection)
}
/// This function returns one goal of the user.
/// Just pass in the username of the user and the goal is returned.
pub fn read_one_goal(data: SearchGoal) -> Result<Vec<Goals>, diesel::result::Error> {
    let connection = &mut establish_connection();
    goals_db::dsl::goals_db::filter(goals_db::table, username.eq(data.username.to_uppercase()))
        .select(Goals::as_returning())
        .get_results(connection)
}
