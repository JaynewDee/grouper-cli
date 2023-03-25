use csv::ReaderBuilder;
use std::{error::Error, io, process};

mod command;

#[derive(Debug, serde::Deserialize)]

struct Input {
    relative_path: String,
    group_size: u8,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StudentRecord {
    student: String,
    #[serde(rename = "Current Score")]
    current_score: String,
}

fn read_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(path)?;

    for result in rdr.deserialize() {
        let record: StudentRecord = result?;
        println!("{:?}", record);
    }

    Ok(())
}

// fn file_exists(path: String) {}

fn main() {
    let args: Vec<String> = command::parse_args().unwrap().collect();
    let relative_path: String = args[1].to_owned();
    let group_size = args[2].parse::<u8>().unwrap_or(5);

    println!("{:?}", relative_path);
    println!("{:?}", group_size);

    let inputs = Input {
        relative_path,
        group_size,
    };
    read_csv(&inputs.relative_path);
}
