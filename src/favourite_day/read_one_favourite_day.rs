use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::FavouriteDay,
    schema::favourite_day::{self, username},
};

/// Returns a result of Favourite day of the actual data that has been queryed from
/// the database.
pub fn read_one_favourite_day(name_of_user: String) -> Result<FavouriteDay, diesel::result::Error> {
    let connection = &mut establish_connection();
    favourite_day::dsl::favourite_day::filter(
        favourite_day::table,
        username.eq(name_of_user.to_uppercase()),
    )
    .select(FavouriteDay::as_returning())
    .get_result(connection)
}
