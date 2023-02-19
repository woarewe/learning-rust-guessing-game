use std::fs;
use std::error::Error;
use std::env;

#[derive(Debug)]
pub enum SearchMode {
    CaseSensitive,
    CaseInsensitive
}

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    search_mode: SearchMode,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let search_mode = match env::var("IGNORE_CASE") {
            Ok(_) => SearchMode::CaseInsensitive,
            Err(_) => SearchMode::CaseSensitive
        };

        let  search_mode = dbg!(search_mode);

        Ok(Self { query, file_path, search_mode })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("{:?}", config.search_mode);

    let results = match config.search_mode {
        SearchMode::CaseSensitive => search(&config.query, &contents),
        SearchMode::CaseInsensitive => search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    let results = dbg!(results);

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_senstivie() {
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
        )
    }
}
