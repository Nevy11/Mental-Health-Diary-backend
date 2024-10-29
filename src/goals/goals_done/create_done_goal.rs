use crate::{
    connection::establish_connection::establish_connection,
    models::{GoalDone, GoalsDone},
    schema::goals_done,
};
use diesel::prelude::*;

/// Create done goal function takes in the username and goal name
/// to be created, returning a result of the GoalsDone struct {id, username and
/// goal name} establishes the connection with the database then makes the query
/// into the database.
pub fn create_done_goal(data: GoalDone) -> Result<GoalsDone, diesel::result::Error> {
    let created_data = GoalDone {
        username: data.username.to_uppercase().clone(),
        goal_name: data.goal_name.clone(),
    };
    let connection = &mut establish_connection();
    diesel::insert_into(goals_done::dsl::goals_done)
        .values(&created_data)
        .returning(GoalsDone::as_returning())
        .get_result(connection)
}
