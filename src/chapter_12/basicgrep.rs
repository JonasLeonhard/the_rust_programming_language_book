///? this chapter contains a basic grep commandline implementation.
///? run with cargo run #QUERY #FILENAME
///? exmpl: cargo run Java quotes.txt
/// for case insensitive grep search: add $env: 
/// export CASE_INSENSITIVE=true  <- sets env variable once 
/// nano ~/.bash_profile <- write permanent env variable here as export above
/// test with: echo $[variable_name] 

/// cargo run > output.txt   <- println to txt file
use std::env::args;
use std::process;

//? import from path
#[path = "./lib.rs"]
mod lib;
use lib::{run, Arguments};

pub fn new() {
    let args: Vec<String> = args().collect(); // get command line args -> [0]= exec name, [...]=argument

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(arguments) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
