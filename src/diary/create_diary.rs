use diesel::prelude::*;

use crate::{connection::establish_connection::establish_connection, models::Diary, schema::diary};

/// This function takes in a struct diary data {username, content} updates the
/// username to uppercase then insets it into the diary database after establishing
/// the connection with the database.
pub fn create_diary(data: Diary) -> Result<Diary, diesel::result::Error> {
    let inserted_values = Diary {
        username: data.username.to_uppercase(),
        content: data.content,
    };
    let connection = &mut establish_connection();
    diesel::insert_into(diary::dsl::diary)
        .values(inserted_values)
        .returning(Diary::as_returning())
        .get_result(connection)
}
