use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Command {
    Add { name: String, department: String },
    GetAll,
    GetBy(String),
}

pub fn loop_init() {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\ninput: \n");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("InputError:"); // red input

        let tokens: Vec<String> = buffer
            .trim()
            .split(" ")
            .map(|token| token.to_lowercase())
            .collect(); // collect input in
        let command: Command;

        if buffer.trim() == "quit" {
            // need to use trim() here because stdin() formats input to
            // have new line character \n at the end : see -> println!("{:?}", buffer);
            break;
        }

        if tokens.len() > 0 {
            match tokens[0].as_ref() {
                // turns String back to &str literal
                "add" =>
                // set command to .clone or .to_string because you cannot move here
                {
                    if tokens.len() == 4 {
                        let d = data.entry(tokens[3].clone()).or_insert(vec![]);
                        d.push(tokens[1].clone());
                    } else {
                        println!("Add Error: format : 'Add x to y'");
                        continue;
                    }
                }
                "get" => {
                    if tokens.len() == 2 {
                        println!("Get All: {:?} {}", data, data.len());
                    } else if tokens.len() == 1 {
                        println!("Get All: ");
                        for (key, val) in data.iter() {
                            println!("--- {} ---", key);
                            for name in val.iter() {
                                println!("{}", name);
                            }
                        }
                    } else {
                        println!("Get Error: format 'Get y' or 'Get");
                        continue;
                    }
                }
                _ => {
                    println!("Input Format Error: try format Add or Get -> eg -> 'add x to y'");
                    continue;
                }
            }
        } else {
            println!("Input Format Error: try format eg -> 'add x to y' or 'get y");
        }
    }
}
