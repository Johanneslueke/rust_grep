use std::fs;
use std::error::Error;


pub fn run(config : Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename).expect("something went wrong");
                      

    println!("Searching for {:?}", &config.query);
    println!("In file {:?}", &config.filename);
   
    println!("With content: \n {}", contents);

    Ok(())
}

pub struct Config{
    pub query : String,
    pub filename : String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok( Config { query, filename } )
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    vec![]
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
}