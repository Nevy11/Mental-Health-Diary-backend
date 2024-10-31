use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::FavouriteDay,
    schema::favourite_day::{self, day_favourite, username},
};

/// This function updates the favourite day's username or actual day depending on the field
/// entered by the user. The name of the user is stored in uppercase while Day is stored
/// without being changed
/// If the function returns None, then the owner has not entered a valid field in the
/// field section.
pub fn update_favourite_day(
    name_of_user: String,
    field: String,
    new_value: String,
) -> Option<Result<FavouriteDay, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let name_of_user = name_of_user.to_uppercase();
    let field = field.to_uppercase();
    let field = field.as_str();
    match field {
        "USERNAME" => Some(
            diesel::update(favourite_day::dsl::favourite_day)
                .filter(username.eq(name_of_user))
                .set(username.eq(new_value.to_uppercase()))
                .returning(FavouriteDay::as_returning())
                .get_result(connection),
        ),
        "DAY_FAVOURITE" => Some(
            diesel::update(favourite_day::dsl::favourite_day)
                .filter(username.eq(name_of_user))
                .set(day_favourite.eq(new_value))
                .returning(FavouriteDay::as_returning())
                .get_result(connection),
        ),
        _ => None,
    }
}
