use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::{Goals, SearchGoal},
    schema::goals_db::{self, goal_name, id},
};

use super::read_goal::read_one_goal;

/// An update, This function only udpates the goal of the user.
pub fn update_goal(
    name_of_the_user: String,
    old_value: String,
    new_value: String,
) -> Option<Result<Goals, diesel::result::Error>> {
    let user_data = SearchGoal {
        username: name_of_the_user.clone().to_uppercase(),
    };
    let goals_available = read_one_goal(user_data);
    match goals_available {
        Ok(goal_data) => {
            for goal in goal_data {
                if old_value == goal.goal_name {
                    let connection = &mut establish_connection();
                    return Some(
                        diesel::update(goals_db::dsl::goals_db)
                            .filter(id.eq(goal.id))
                            .set(goal_name.eq(new_value))
                            .returning(Goals::as_returning())
                            .get_result(connection),
                    );
                }
            }
            None
        }
        Err(e) => Some(Err(e)),
    }
}
