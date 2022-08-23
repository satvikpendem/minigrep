use std::{env, error::Error, fs::read_to_string};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            filename,
            ignore_case,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let Self {
            query,
            filename,
            ignore_case,
        } = self;

        let contents = read_to_string(filename)?;

        let results = if *ignore_case {
            search(query, &contents)
        } else {
            search_case_insensitive(query, &contents)
        };

        for line in results {
            println!("{}", line);
        }

        Ok(())
    }
}

fn _search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            if ignore_case {
                line.to_lowercase().contains(&query.to_lowercase())
            } else {
                line.contains(&query)
            }
        })
        .collect()
}

#[must_use]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    _search(query, contents, false)
}

#[must_use]
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    _search(query, contents, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
