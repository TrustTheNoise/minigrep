use colored::*;
use std::{fs,env};
use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(&config.file_path)?;

    println!("With text:\n{contents}");

    let finding_lines = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    println!("\n");

    out_of_string(&finding_lines, config);

    Ok(())
}

fn out_of_string(finding_lines: &Vec<&str>, config: &Config)
{
    if config.ignore_case
    {
        for lines in finding_lines
        {
            for words in lines.split_whitespace()
            {
                if words.to_string().to_lowercase().contains(&config.query)
                {
                    print!("{} ", words.blue());
                    continue;
                }
                print!("{words} ");
            }
            println!();
        }
    }else
    {
        for lines in finding_lines
        {
            for words in lines.split_whitespace()
            {
                if words.to_string().contains(&config.query)
                {
                    print!("{} ", words.blue());
                    continue;
                }
                print!("{words} ");
            }
            println!();
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines()
    {
        if line.contains(query)
        {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)
        {
            result.push(line);
        }
    }
    result
}

pub struct Config
{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}