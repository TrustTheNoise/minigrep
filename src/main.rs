use std::{process, env};
use minigrep::Config;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let conf = Config::build(&args).unwrap_or_else(|err| 
        {
            println!("Error: {err}");
            process::exit(1);
        });

    println!("Searching for {}", conf.query);
    println!("In file {}", conf.file_path);

    if let Err(e) = minigrep::run(&conf)
    {
        println!("Error: {e}");
        process::exit(1);
    };
}