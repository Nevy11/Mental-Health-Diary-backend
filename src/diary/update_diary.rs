use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    models::Diary,
    schema::diary::{self, content, username},
};

/// This function takes in the username, field and new value.
/// username is used to search for the available data in the database.
/// Field => What to update and finally new_value that is used to be updated.
/// If a field does not match to username or Content, It returns None.
pub fn update_diary(
    name_of_user: String,
    field: String,
    new_value: String,
) -> Option<Result<Diary, diesel::result::Error>> {
    let connection = &mut establish_connection();
    let name_of_user = name_of_user.to_uppercase();
    let name_of_user = name_of_user.as_str();
    let field = field.to_uppercase();
    let field = field.as_str();
    match field {
        "USERNAME" => Some(
            diesel::update(diary::dsl::diary)
                .filter(username.eq(name_of_user))
                .set(username.eq(new_value.to_uppercase()))
                .returning(Diary::as_returning())
                .get_result(connection),
        ),
        "CONTENT" => Some(
            diesel::update(diary::dsl::diary)
                .filter(username.eq(name_of_user))
                .set(content.eq(new_value))
                .returning(Diary::as_returning())
                .get_result(connection),
        ),
        _ => None,
    }
}
