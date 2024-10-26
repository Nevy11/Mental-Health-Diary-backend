use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection, models::Goals, schema::goals_db,
};

pub fn create_goal(data: Goals) -> Result<Goals, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::insert_into(goals_db::dsl::goals_db)
        .values(data)
        .returning(Goals::as_returning())
        .get_result(connection)
}
