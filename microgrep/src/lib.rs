// Contains the Logic Part of the Project
use std::error::Error;
use std::fs;
use std::env;

// Config Struct
pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config{
    // this takes in the Args and Returns a Result containing Config / Err
    pub fn build(args: &[String]) -> Result<Config,&'static str>{ // Error must always be static &str
        if args.len() < 3{
            return Err("Not Enough Arguments")
        }

        let query = args[1].clone(); 
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok(); // checks if the env var is set, if not returns false
        Ok(Config{query, file_path,ignore_case})
    }
}

// run fn takes in config and prints the contents of the file and returns a result
// if no errors, it returns Unit(), else it returns an Error
// Box<dyn Error> means the function will return a type that implements the Error trait, 
// but we don’t have to specify what particular type the return value will be. 
// This gives us flexibility to return error values that may be of different types in different error cases.
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?; // "?" for error propagation
    //println!("Contents of File: \n{contents}\n");

    println!("\nThe following lines contain the word {}:",&config.query);
    
    let result = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in result{
        println!("{line}")
    }
    Ok(())
}

// a bare minimum fn for our first test, we know it fails and then we update it
// A lifetime 'a is used to let Rust know that the Output lives as long as "content" lives
// this is because we get the output from the variable "content"
// pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
//     vec![]
// }

// search fn returns all the lines that contain the query word
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = vec![];
    for line in contents.lines(){ // returns an iterator that iterates line by line
        if line.contains(query){ // checks if it contains something, it checks letter by letter, so "hello".contains("ll") returns true
        results.push(line);
        }
    }
    results
}

// .to_lowercase() returns a String type and ".contains()" need an type that implements "Pattern Train"
// so after query.to_lowercase(), we get a String, which we are converting it to &String
// due to coercion &String will be converted to &str which implement the "Pattern Trait"
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = vec![];
    let query = query.to_lowercase(); // to_lowercase() handles basic unicode and not 100% accurate
    for line in contents.lines(){ // returns an iterator that iterates line by line
        if line.to_lowercase().contains(&query){ // checks if it contains something, it checks letter by letter, so "hello".contains("ll") returns true
        results.push(line);
        }
    }
    results
}


// Test Driven Development
/*
1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!
*/

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_search_case_sensitive(){
        let query = "duct";
        // "\" tells rust not to put a newline char at beginning of contents of this string literal
        let contents = "\
Rust:
safe, fast, productive.
Duct typing is necessary
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query,contents))
    }

    #[test]
    fn test_search_case_insensitive(){
        let query = "rUsT";
        // "\" tells rust not to put a newline char at beginning of contents of this string literal
        let contents = "\
Rust:
safe, fast, productive.
Trust is needed
Pick three.";

        assert_eq!(
            vec!["Rust:","Trust is needed"], 
            search_case_insensitive(query,contents)
        );
    }
}