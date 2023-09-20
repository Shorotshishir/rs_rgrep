use std::{env, process};

use rs_rgrep::Config;

fn main() {
    println!("Hi ! this is rgrep!");
    let args:  Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments {err}");
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("In File {}",config.file_path);

    if let Err(e) = rs_rgrep::run(config) {
        println!("Application Error ! {e}");
        process::exit(1);
    }
}
