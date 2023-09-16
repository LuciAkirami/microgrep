/*
    microgrep arguement
    query -> the word to search for
    file_path -> The file in which the word to search for
    ignore_case -> If True, let it run without case sensitive else oppsite

    Example with cargo run, in Powershell/Command Prompt:
    Without Case Sensitive Search
    $Env:IGNORE_CASE=1; cargo run -- to poem.txt -> will search for "to" in "poem.txt"

    With Case Sensitive Search
    Remove-Item Env:IGNORE_CASE
    cargo run to poem.txt -> will search for "to" in "poem.txt"

    Example with binary, in Powershell/Command Prompt:
    Without Case Sensitive Search
    $Env:IGNORE_CASE=1; 
    .\target\debug\microgrep.exe to poem.txt -> will search for "to" in "poem.txt"

    With Case Sensitive Search
    Remove-Item Env:IGNORE_CASE
    .\target\debug\microgrep.exe to poem.txt -> will search for "to" in "poem.txt"


    
*/
// #![allow(unused)]
// use std::error::Error;
// use std::fs;
use std::env;
use std::process;
use microgrep::Config; // microgrip is the project name and the fns are from the lib.rs

fn main() {
    let arg_list: Vec<String> = env::args().collect();
    // args() returns a iterator containing arguments passed in the terminal
    // collect() func will change that iterator to a collections type like a vector
    // as the arguments will be text, we defined the type as Vec<String>
    // we need to explicitly annotate the type else rust panics

    // Note: the first arguement(0th Index) will always be the path to the binary
    // hence arg_list[0] = target/debug/microgrep.exe or target\\debug\\microgrep.exe

    // println!("{:?}",arg_list);
    // dbg!(arg_list);

    // Reading the file - read_to_string() -> returns a Result<String, Err>
    // If successfull, stored in var file_contents, if Err, then expect() message printed
    // let file_contents = fs::read_to_string(file_path).expect("Should have read a files");

    // let config = match Config::build(&arg_list) {
    //     Err(k) => {
    //         println!("Problem Parsing Arguments: {k}"); // print error and exit if error
    //         process::exit(1); // will shutdown the execution immediately
    //     },
    //     Ok(k) => k // create an instance of Config and assign to config is everything OK
    // };
    // this can be written in more simpler way as below
    // unwrap_or_else -> Returns the contained Ok value or computes it from a closure.
    // a closure is similar to a lambda(lambda x: some_operation) fn in Python
    // if Err is returned, the value inside it is passed to "err" in |err|
    let config = Config::build(&arg_list).unwrap_or_else(|err| {
        // eprintln macro prints the info to the stderr rather than stdout
        eprintln!("Problem Parsing Arguments: {err}");
        process::exit(1)
    });

    println!("Searching for '{}'",config.query);
    println!("In the file {}",config.file_path);

    if let Err(e) = microgrep::run(config){
        eprintln!("Application Error: {}",e);
        process::exit(1)
    }



}

