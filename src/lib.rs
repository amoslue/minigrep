use std::error::Error;
use std::fs;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
}
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    let contents = fs::read_to_string(config.file_path)?;

    let ret = if config.ignore_case{
        search(&config.query, &contents) 
    } else{
search_case_insensitive(&config.query, &contents)
    };

    for line in ret{
        println!("{line}");
    }
      
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    for line in content.lines() {
        if line.contains(query){
            ret.push(line);
        }
    }
        ret   
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str)->Vec<&'a str>{
    let mut ret:Vec<&str> = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            ret.push(line);
        }
    }
    ret
}
