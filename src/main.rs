use csv::ReaderBuilder;
use std::{error::Error, io, process};

mod command;

#[derive(Debug, serde::Deserialize)]

struct Input {
    filepath: String,
    group_size: u8,
}

#[derive(Debug, serde::Deserialize)]
struct StudentRecord {
    Student: String,
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path("test-bcs3.csv")?;

    for result in rdr.deserialize() {
        let record: StudentRecord = result?;
        println!("{:?}", record);
    }

    Ok(())
}

// fn file_exists(path: String) {}

fn main() {
    read_csv().unwrap();
    let args = command::parse_args();
    for arg in args.unwrap() {
        println!("{:?}", arg);
    }
}
