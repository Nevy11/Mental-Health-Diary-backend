use diesel::prelude::*;
use diesel::result::Error;

use crate::connection::establish_connection::establish_connection;
use crate::models::NewRecord;
use crate::schema;

pub fn insert_data_in_batches(data: Vec<NewRecord>, batch_size: usize) -> Result<(), Error> {
    let connection = &mut establish_connection();
    for chunk in data.chunks(batch_size) {
        diesel::insert_into(schema::questions::table)
            .values(chunk)
            .execute(connection)?;
    }
    Ok(())
}
