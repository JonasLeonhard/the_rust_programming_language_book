use std::error::Error;
use std::{fs, env};

pub fn run(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let file_contents =
        fs::read_to_string(String::from("./src/chapter_12/") + arguments.filename.as_str())?;

    let results = if arguments.search_case_insensitive {
        search_case_insensitive(&arguments.query, &file_contents)
    } else {
        search(&arguments.query, &file_contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Arguments {
    pub query: String,
    pub filename: String,
    pub search_case_insensitive : bool,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() != 3 {
            // panic!("wrong number of arguments: try -> cargo run #QUERY #FILENAME");
            return Err("wrong number of arguments: try -> cargo run #QUERY #FILENAME -> eg cargo run Java quotes.txt");
        }

        Ok(Arguments {
            query: args[1].clone(),
            filename: args[2].clone(),
            search_case_insensitive : env::var("CASE_INSENSITIVE").is_err()
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            matching.push(line);
        }
    }

    matching
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(query.to_lowercase().as_str()) {
            matching.push(line);
        }
    }

    matching
}

#[cfg(test)]
mod tests {
    use super::*; //? Import all from super
    #[test]
    fn basicgrep_search() {
        let query = "Java";
        let contents = "\nQuote:\nJava is to JavaScript what Car is to Carpet.\n- Chris Heilmann";

        assert_eq!(vec!["Java is to JavaScript what Car is to Carpet."], search(query, contents));
    }

    #[test]
    fn basicgrep_search_case_insensitive() {
        let query = "java";
        let contents = "\nQuote:\nJava is to JavaScript what Car is to Carpet.\n- Chris Heilmann";

        assert_eq!(vec!["Java is to JavaScript what Car is to Carpet."], search_case_insensitive(query, contents));
    }
}
