use diesel::prelude::*;

use crate::{
    connection::establish_connection::establish_connection, models::SignUp, schema::chat_users,
};

/// this method takes in the username and password, then creates the user's data
/// to the system.
/// it connects with the system returning a result of ChatUsers for error handling.
#[allow(dead_code, unused_variables)]
pub fn create_chat_user(data: SignUp) -> Result<SignUp, diesel::result::Error> {
    #[allow(unused_variables)]
    let connection = &mut establish_connection();
    diesel::insert_into(chat_users::dsl::chat_users)
        .values(data)
        .returning(SignUp::as_returning())
        .get_result(connection)
}
