use std::env::args;
pub struct Cli;

impl Cli {
    pub fn parse_args() -> Vec<String> {
        let args = args();
        println!("{:?}", args);
        args.collect()
    }
}
