use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::FavouriteDay,
    schema::favourite_day::{self, username},
};

/// Takes in the name of the user, searches it through the database returning the
/// actual value of the user which has already been removed from the database.
pub fn delete_favourite_day(
    name_of_the_user: String,
) -> Result<FavouriteDay, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(favourite_day::dsl::favourite_day)
        .filter(username.eq(name_of_the_user.to_uppercase()))
        .returning(FavouriteDay::as_returning())
        .get_result(connection)
}
