use std::env;
use std::io::{self};
use std::process::exit;

use lib::run;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    match run(args, &mut io::stdout()) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }
    }
}
