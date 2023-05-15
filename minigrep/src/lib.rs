use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config{
    // Note: the new constructor is always an helper function!!
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // is_err() just returns a bool if the result is an error
        Ok(Config{ query, filename, case_sensitive: env::var("CASE_SENSITIVE").is_err() })
    }

}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(&config.filename)?;


    if config.case_sensitive{
        for line in search(&config.query, &contents){
            println!("{}", line);
        }
    }
    else{
        for line in search(&config.query, &contents){
            println!("{}", line);
        }
    }

    Ok(())
}


pub fn search<'a>(query: &str, content:&'a str) -> Vec<&'a str>{

    /*let mut res = vec![];

    for line in content.lines(){
       if line.contains(query){
            res.push(line);
       }
    }

    res*/

    content
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}


pub fn search_case_insensitive<'a>(query: &str, content:&'a str) -> Vec<&'a str>{

    let query = query.to_lowercase();
    let mut res = vec![];

    for line in content.lines(){
       if line.to_lowercase().contains(&query){
            res.push(line);
       }
    }

    res
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }

    #[test]
    fn two_result(){
        let query = "Ano";
        let contents = "Mi piace prenderlo
nell'aNo";

        assert_eq!(vec!["nell'aNo"], search_case_insensitive(&query, &contents));
    }

}
