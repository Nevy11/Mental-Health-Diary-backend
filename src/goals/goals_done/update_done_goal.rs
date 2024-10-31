use crate::{
    connection::establish_connection::establish_connection,
    models::{GoalsDone, SearchGoal},
    schema::goals_done::{self, goal_name, id},
};
use diesel::prelude::*;

use super::read_done_goal::read_one_done_goal;

/// This function updates the done struct
/// Takes in the username, old_value and new_value.
/// stores the username in a searchgoal
/// reads one data from the username.
/// loops over the data in the goals to obtain each goal.
/// if the old value matches one, it updates the new
pub fn update_done_goal(
    name_of_the_user: String,
    old_value: String,
    new_value: String,
) -> Option<Result<GoalsDone, diesel::result::Error>> {
    let user_data = SearchGoal {
        username: name_of_the_user.clone().to_uppercase(),
    };
    let goals_available = read_one_done_goal(user_data);
    match goals_available {
        Ok(goal_data) => {
            for goal in goal_data {
                if old_value == goal.goal_name {
                    let connection = &mut establish_connection();
                    return Some(
                        diesel::update(goals_done::dsl::goals_done)
                            .filter(id.eq(goal.id))
                            .set(goal_name.eq(new_value))
                            .returning(GoalsDone::as_returning())
                            .get_result(connection),
                    );
                }
            }
            println!("The old data doesn't match the new value");
            None
        }
        Err(e) => Some(Err(e)),
    }
}
