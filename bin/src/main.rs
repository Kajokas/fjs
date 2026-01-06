use crate::opts::*;
use std::{env};

pub mod utils;
pub mod opts;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No arguments provided");
        return;
    }

    match args[1].as_str() {
        "init" => init::init(args),
        "run" => run::run(),
        "build" => {
            let extra = if args.len() >= 3 {
                Some(args[2].clone())
            } else {
                None
            };
            build::build(extra)
        },
        _ => println!("No such command"),
    }
}
