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
    pub fn new(mut args: std::env::Args) -> Result<Arguments, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Arguments {
            query: query,
            filename: filename,
            search_case_insensitive : env::var("CASE_INSENSITIVE").is_err()
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut matching = vec![];

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         matching.push(line);
    //     }
    // }

    // matching

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut matching = vec![];

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(query.to_lowercase().as_str()) {
    //         matching.push(line);
    //     }
    // }

    // matching

    contents.lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
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
