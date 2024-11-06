use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::AiReturn,
    schema::ai::{self, username},
};

/// This function takes in the name of the user, searches it in the database, then
/// deletes all the data that is stored in the database.
pub fn delete_ai(name_of_user: String) -> Result<Vec<AiReturn>, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(ai::dsl::ai)
        .filter(username.eq(name_of_user.to_uppercase()))
        .returning(AiReturn::as_returning())
        .get_results(connection)
}
