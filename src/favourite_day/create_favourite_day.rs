use crate::{
    connection::establish_connection::establish_connection, models::FavouriteDay,
    schema::favourite_day,
};
use diesel::prelude::*;

/// This function takes in a struct with username and day_favourite into it, updates the username
/// to uppercase, then inserts the data to the favourite day table returning a result of the
/// favourite day.
pub fn create_favourite_day(data: FavouriteDay) -> Result<FavouriteDay, diesel::result::Error> {
    let connection = &mut establish_connection();

    let updated_value = FavouriteDay {
        username: data.username.to_uppercase(),
        day_favourite: data.day_favourite,
    };
    diesel::insert_into(favourite_day::dsl::favourite_day)
        .values(updated_value)
        .returning(FavouriteDay::as_returning())
        .get_result(connection)
}
