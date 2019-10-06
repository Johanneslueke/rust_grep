use std::fs;
use std::env;
use std::error::Error;


pub fn run(config : Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;
    
    let results = if config.case_sensitive 
    {
        search(&config.query, &contents)
    } 
    else
    {
        search_case_insensitiv(&config.query, &contents)
    };

    for line in results {
        println!("{}",line);
    }
    Ok(())
}

pub struct Config{
    pub query : String,
    pub filename : String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); //consume binary name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        let case_sensitive = env::var("CASE_INSENSITIV").is_err();
        Ok( Config { query, filename, case_sensitive } )
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitiv<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter( |line| line.to_lowercase().contains(query) )
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";

        let contents ="\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query,contents)
        );
    }

    #[test]
    fn case_insensitive(){
        let query = "duct";

        let contents ="\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitiv(query,contents)
        );
    }
}