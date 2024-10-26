use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::Goals,
    schema::goals_db::{self, goal_name},
};

pub fn delete_goal(data: Goals) -> Result<Goals, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(goals_db::dsl::goals_db)
        .filter(goal_name.eq(data.goal_name.to_uppercase().clone()))
        .returning(Goals::as_returning())
        .get_result(connection)
}
