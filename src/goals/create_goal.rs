use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::{Goal, Goals},
    schema::goals_db,
};
/// This function takes in a goal and the username of the person who has updated that goal,
/// updates the database the returns the Goal struct (a result of that struct)
pub fn create_goal(data: Goal) -> Result<Goals, diesel::result::Error> {
    let created_data = Goal {
        username: data.username.to_uppercase().clone(),
        goal_name: data.goal_name.clone(),
    };
    let connection = &mut establish_connection();
    diesel::insert_into(goals_db::dsl::goals_db)
        .values(&created_data)
        .returning(Goals::as_returning())
        .get_result(connection)
}
