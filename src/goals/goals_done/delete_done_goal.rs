use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::{GoalDone, GoalsDone},
    schema::goals_done::{self, goal_name, username},
};

/// This function deletes done goals: {username, goal name} => Returning a result of the
/// deleted data.
pub fn delete_done_goal(data: GoalDone) -> Result<GoalsDone, diesel::result::Error> {
    let deleted_data: GoalDone = GoalDone {
        username: data.username.to_uppercase().clone(),
        goal_name: data.goal_name.clone(),
    };
    let connection = &mut establish_connection();
    diesel::delete(goals_done::dsl::goals_done)
        .filter(username.eq(deleted_data.username.clone()))
        .filter(goal_name.eq(deleted_data.goal_name.clone()))
        .returning(GoalsDone::as_returning())
        .get_result(connection)
}

/// This function deletes all data of the user when the user presses the clear button
pub fn delete_all_done_goals(
    name_of_user: String,
) -> Result<Vec<GoalsDone>, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(goals_done::dsl::goals_done)
        .filter(username.eq(name_of_user.to_uppercase().clone()))
        .returning(GoalsDone::as_returning())
        .get_results(connection)
}
