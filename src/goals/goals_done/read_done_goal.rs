use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::{GoalsDone, SearchGoal},
    schema::goals_done::{self, username},
};

/// The function returns all deleted data.
pub fn read_all_done_goal() -> Result<Vec<GoalsDone>, diesel::result::Error> {
    let connection = &mut establish_connection();
    goals_done::dsl::goals_done::load::<GoalsDone>(goals_done::table, connection)
}

/// The function takes in the username of the person returning a result of vector of done goals.
pub fn read_one_done_goal(data: SearchGoal) -> Result<Vec<GoalsDone>, diesel::result::Error> {
    let connection = &mut establish_connection();
    goals_done::dsl::goals_done::filter(
        goals_done::table,
        username.eq(data.username.to_uppercase()),
    )
    .select(GoalsDone::as_returning())
    .get_results(connection)
}
