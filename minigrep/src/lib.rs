use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3{
            return Err("Not enough arguments.");
        }

        let query: String = args[1].clone();
        let file: String = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query,file,case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file)?;
    let result = if config.case_sensitive{
        search(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query, &contents)
    };

    for lines in result {
        println!("{}",lines);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "dolor";
        let contents = "Lorem 
ipsum
dolor
DoloR
sit   
amet";
        assert_eq!(vec!["dolor"], search(query,contents));
    }
    
    #[test]
    fn case_insensitive(){ 
        let query = "DoLoR";
        let contents = "Lorem 
ipsum
dolor
sit   
amet
DOLOR
DolOR
doLOR";
        assert_eq!(vec!["dolor","DOLOR","DolOR","doLOR"], search_case_insensitive(query,contents));
    }
}
