use std::env;
use std::io::{self};
use std::process::exit;

use lib::run::run;

pub mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    match run(args, &mut io::stdout()) {
        Ok(()) => {}
        Err(_) => {
            exit(1);
        }
    }
}
