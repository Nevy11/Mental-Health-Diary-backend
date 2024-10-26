use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::Goals,
    schema::goals_db::{self, goal_name},
};

pub fn read_all_goal() -> Result<Vec<Goals>, diesel::result::Error> {
    let connection = &mut establish_connection();
    goals_db::dsl::goals_db::load::<Goals>(goals_db::table, connection)
}

pub fn read_one_goal(data: Goals) -> Result<Goals, diesel::result::Error> {
    let connection = &mut establish_connection();
    goals_db::dsl::goals_db::filter(goals_db::table, goal_name.eq(data.goal_name))
        .select(Goals::as_returning())
        .get_result(connection)
}
