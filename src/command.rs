use std::env::{args, Args};

pub fn parse_args() -> Result<Args, ()> {
    Ok(args())
}
