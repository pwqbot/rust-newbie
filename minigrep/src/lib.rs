use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        return Ok(Config {
            query,
            filename,
            ignore_case,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vecs = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            vecs.push(line)
        }
    }
    return vecs;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut vecs = Vec::new();

    for line in contents.lines() {
        println!("{line}");
        if line.to_lowercase().contains(&query) {
            vecs.push(line)
        }
    }
    return vecs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\n\
            safe, fast, productive.\n\
            Pick three.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:\n\
            safe, fast, prodective.\n\
            Pick three.";
        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }
}
