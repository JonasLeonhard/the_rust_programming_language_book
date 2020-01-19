///? Error handling - chapter 9
///? with backtrace: "RUST_BACKTRACE=1 cargo run"
use std::fs::File;
use std::io::ErrorKind;

pub fn error() {
    println!("called Error module:");
    // panic!("CRASH AND BURN!");

    let file = File::open("error.txt");
    let file = match file {
        Ok(data) => data,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./src/chapter_9/error.txt") {
                Ok(created) => created,
                Err(err) => panic!("Error creating file {}", err),
            },
            any => {
                panic!("Error finding file {:?}", any);
            }
        },
    };

    //*! Result<T,E> accepts closures:
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    println!("{:?}", file);
}