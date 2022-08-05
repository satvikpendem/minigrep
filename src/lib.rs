use std::{env, error::Error, fs::read_to_string};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let ignore_case = env::var("CASE_SENSITIVE").is_ok();

        Ok(Self {
            query,
            filename,
            ignore_case,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let Config {
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
    let mut results: Vec<&str> = Vec::new();
    contents.lines().for_each(|line| {
        if ignore_case {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        } else if line.contains(&query) {
            results.push(line);
        }
    });
    results
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
    use std::string::ToString;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn parses_args() {
        let args = ["minigrep", "hello", "poem.txt"];
        Config::new(&args.map(ToString::to_string)).unwrap();
    }

    #[test]
    #[should_panic]
    #[allow(clippy::unwrap_used)]
    fn not_enough_args() {
        let args = ["minigrep", "hello"];
        Config::new(&args.map(ToString::to_string)).unwrap();
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn file_not_found() {
        let args = ["minigrep", "hello", "unknownfile.txt"];
        Config::new(&args.map(std::string::ToString::to_string)).unwrap();
    }

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
