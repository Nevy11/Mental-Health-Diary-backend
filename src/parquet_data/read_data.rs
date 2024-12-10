use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use std::fs::File;

pub(crate) fn read_parquet_file(file_path: &str) -> Vec<(String, String)> {
    let file = File::open(file_path).expect("Could not open file");
    let reader = SerializedFileReader::new(file).expect("Could not read parquet file");
    let iter = reader
        .get_row_iter(None)
        .expect("Could not get row iterator");

    let mut data = Vec::new();

    for row_result in iter {
        match row_result {
            Ok(record) => {
                // Use column indices instead of names
                let question = record
                    .get_string(0)
                    .unwrap_or(&String::from(""))
                    .to_string();
                let context = record
                    .get_string(1)
                    .unwrap_or(&String::from(""))
                    .to_string();
                data.push((question, context));
            }
            Err(e) => {
                eprintln!("Error reading row: {:?}", e); // Optionally handle the error
            }
        }
    }

    data
}
