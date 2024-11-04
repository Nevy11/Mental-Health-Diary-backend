use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::Diary,
    schema::diary::{self, username},
};

/// This function takes in the name of the user, returning a diary content data.
pub fn read_one_diary_content(name_of_user: String) -> Result<Diary, diesel::result::Error> {
    let connection = &mut establish_connection();
    diary::dsl::diary::filter(diary::table, username.eq(name_of_user.to_uppercase()))
        .select(Diary::as_returning())
        .get_result(connection)
}

/// Thsi function returns all the username and data in the database.
pub fn read_all_diary_content() -> Result<Vec<Diary>, diesel::result::Error> {
    let connection = &mut establish_connection();
    diary::dsl::diary::load::<Diary>(diary::table, connection)
}
