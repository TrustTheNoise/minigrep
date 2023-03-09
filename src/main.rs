
use std::{process, env};
use minigrep::Config;

fn main() {

    let conf = Config::build(env::args()).unwrap_or_else(|err| 
        {
            eprintln!("Error: {err}");
            process::exit(1);
        });

    println!("Searching for {}", conf.query);
    println!("In file {}", conf.file_path);

    if let Err(e) = minigrep::run(&conf)
    {
        eprintln!("Error: {e}");
        process::exit(1);
    };
}