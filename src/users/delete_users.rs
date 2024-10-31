use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection,
    favourite_day::delete_favourite_day::delete_favourite_day,
    models::ChatUsers,
    schema::chat_users::{self, username},
};

pub fn delete_chat_user(name_of_user: String) -> Result<ChatUsers, diesel::result::Error> {
    let deleted_result = delete_favourite_day(name_of_user.clone());
    match deleted_result {
        Ok(deleted_data) => {
            println!("Deleted: {:?}", deleted_data.username)
        }
        Err(e) => {
            println!("Error: {e:?}")
        }
    }
    let connection = &mut establish_connection();
    diesel::delete(chat_users::dsl::chat_users)
        .filter(username.eq(name_of_user.to_uppercase()))
        .returning(ChatUsers::as_returning())
        .get_result(connection)
}
