use std::error::Error;
use std::{fs};

pub fn run(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let file_contents =
        fs::read_to_string(String::from("./src/chapter_12/") + arguments.filename.as_str())?;

    println!("filename: {}", arguments.filename);
    println!("basicgrep: {:?}", file_contents);

    Ok(())
}

pub struct Arguments {
    query: String,
    filename: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() != 3 {
            // panic!("wrong number of arguments: try -> cargo run #QUERY #FILENAME");
            return Err("wrong number of arguments: try -> cargo run #QUERY #FILENAME");
        }
        Ok(Arguments {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}