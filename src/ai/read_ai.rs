use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::AiReturn,
    schema::ai::{self, username},
};

/// Takes in the name of the user, returning all the values in the database of that
/// particular person. This is after it has established connection with the database.
pub fn read_one_ai(name_of_user: String) -> Result<Vec<AiReturn>, diesel::result::Error> {
    let connection = &mut establish_connection();
    ai::dsl::ai::filter(ai::table, username.eq(name_of_user.to_uppercase()))
        .select(AiReturn::as_returning())
        .get_results(connection)
}

/// This function reads all the values stored in the database.
/// The function returns a result of a vector of AiReturn struct.
pub fn read_all_ai() -> Result<Vec<AiReturn>, diesel::result::Error> {
    let connection = &mut establish_connection();
    ai::dsl::ai::load::<AiReturn>(ai::table, connection)
}
