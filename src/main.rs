use std::{env, process};

use rs_rgrep::Config;

fn main() {
    let args:  Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments {err}");
        process::exit(1);
    });

    if let Err(e) = rs_rgrep::run(config) {
        println!("Application Error ! {e}");
        process::exit(1);
    }
}
