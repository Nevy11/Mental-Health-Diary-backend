use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::{Goal, Goals},
    schema::goals_db::{self, goal_name, username},
};

/// This function establishes the connection with the database, searches the data from the
/// database, if it founds, it returns the data and deletes it, but if it fails, it returns
/// an error.
pub fn delete_goal(data: Goals) -> Result<Goals, diesel::result::Error> {
    let deleted_data = Goal {
        username: data.username.to_uppercase().clone(),
        goal_name: data.goal_name.clone(),
    };
    let connection = &mut establish_connection();
    diesel::delete(goals_db::dsl::goals_db)
        .filter(username.eq(deleted_data.username.clone()))
        .filter(goal_name.eq(deleted_data.goal_name.to_uppercase().clone()))
        .returning(Goals::as_returning())
        .get_result(connection)
}

/// This function deletes all data of the user when the user presses the clear button
pub fn delete_all_goals(name_of_user: String) -> Result<Goals, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(goals_db::dsl::goals_db)
        .filter(username.eq(name_of_user.to_uppercase().clone()))
        .returning(Goals::as_returning())
        .get_result(connection)
}
