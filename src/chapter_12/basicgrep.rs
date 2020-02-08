///? this chapter contains a basic grep commandline implementation.
///? run with cargo run #QUERY #FILENAME
use std::env::args;
use std::{process};

//? import from path
#[path = "./lib.rs"] mod lib;
use lib::{Arguments, run};

pub fn new() {
    let args: Vec<String> = args().collect(); // get command line args -> [0]= exec name, [...]=argument

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(arguments){
        println!("Application error: {}", e);
        process::exit(1);
    }
}
