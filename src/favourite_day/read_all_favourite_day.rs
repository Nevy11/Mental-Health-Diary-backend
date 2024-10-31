use crate::{
    connection::establish_connection::establish_connection, models::FavouriteDay,
    schema::favourite_day,
};
use diesel::prelude::*;

/// Reads all the value that is available in the database.
/// It retuns a vector of the favouriteDay struct.
pub fn read_all_favourite_day() -> Result<Vec<FavouriteDay>, diesel::result::Error> {
    let connection = &mut establish_connection();
    favourite_day::dsl::favourite_day::load::<FavouriteDay>(favourite_day::table, connection)
}
