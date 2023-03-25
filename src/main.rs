mod command;

fn main() {
    let args = command::parse_args();
    println!("{:?}", args);
}
