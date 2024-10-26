use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::Goals,
    schema::goals_db::{self, goal_name, username},
};

pub fn update_goal(
    name_of_the_user: String,
    field: String,
    new_value: String,
) -> Option<Result<Goals, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let field = field.to_uppercase();
    let field = field.as_str();
    match field {
        "GOAL_NAME" => Some(
            diesel::update(goals_db::dsl::goals_db)
                .filter(username.eq(name_of_the_user.to_uppercase()))
                .set(goal_name.eq(new_value))
                .returning(Goals::as_returning())
                .get_result(connection),
        ),
        "USERNAME" => Some(
            diesel::update(goals_db::dsl::goals_db)
                .filter(username.eq(name_of_the_user.to_uppercase()))
                .set(username.eq(new_value.to_uppercase()))
                .returning(Goals::as_returning())
                .get_result(connection),
        ),
        _ => None,
    }
}
