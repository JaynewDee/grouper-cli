mod command;

use command::Cli;
use grouper_lib::{FileHandler, Utils};

fn main() {
    let handler = FileHandler::new();
    let files = handler.read_directory().unwrap();
    println!("{:?}", files);
    let args: Vec<String> = Cli::parse_args();

    println!("{:?}", args);

    let data = build_groups("KU FSF 2022.json", 4).unwrap();
    let groups = &data[0];

    println!("GROUPS\n{}", groups);
}

fn build_groups(obj_name: &str, group_size: u16) -> Result<Vec<String>, ()> {
    let test_sd = 2;
    let handler = FileHandler::new();

    let students = handler
        .read_and_return_students(obj_name)
        .expect("Function read_and_return_students should have returned a vector of students.");

    let balanced = Utils::multi_balance(4, students, group_size, test_sd);

    let groups_json =
        Utils::treemap_to_json(balanced).expect("Failed to parse json from groups map ... ");

    Ok(vec![groups_json])
}
