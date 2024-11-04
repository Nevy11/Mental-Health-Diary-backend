use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::Diary,
    schema::diary::{self, username},
};

/// This function takes in the name of the user, searches it if it is available in
/// the database. If it is found, it deletes the data but if not it returns an error.
pub fn delete_diary(name_of_user: String) -> Result<Diary, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::delete(diary::dsl::diary)
        .filter(username.eq(name_of_user.to_uppercase()))
        .returning(Diary::as_returning())
        .get_result(connection)
}
