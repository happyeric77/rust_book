use std::process;
use std::env;
use std::fs;

struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        if (args.len() <2) {
            return Err(String::from("Parse args error, please input at least two arguments\n"))
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        let config = Config {query, filename, case_sensitive};
        Ok(config)
    }
}     

pub fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args)?;
    let content = fs::read_to_string(format!("./{}", config.filename)).unwrap_or_else(|e|{
        print!("Error occurs (read_file): {}", e);
        process::exit(1);
    });
    let search_result: Vec<&str>;
    if config.case_sensitive == true {
        search_result = search_case_sensitive(&config.query, &content);
    } else {
        search_result = search_case_insensitive(&config.query, &content);
    }
    print!("Finalized version: {:?}\n\n", search_result);
    Ok(())
}

fn search_case_sensitive<'a>(query: &String, content: &'a String) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &String, content: &'a String) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query[..].to_lowercase()) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_result_case_sensitive() {
        let query = String::from("ColorfulLife");
        let content = String::from("
Today is a good day.
Hello ColorfulLife.
Hello colorfullife.
        ");
        assert_eq!(vec!["Hello ColorfulLife."], search_case_sensitive(&query, &content));
    }
    #[test]
    fn get_result_case_insensitive() {
        let query = String::from("ColorfulLife");
        let content = String::from("
Today is a good day.
Hello ColorfulLife.
Hello colorfullife.
        ");
        assert_eq!(vec!["Hello ColorfulLife.", "Hello colorfullife."], search_case_insensitive(&query, &content));
    }
}