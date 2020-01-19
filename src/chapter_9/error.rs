///? Error handling - chapter 9
///? with backtrace: "RUST_BACKTRACE=1 cargo run"
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
pub fn error() {
    println!("called Error module:");
    // panic!("CRASH AND BURN!");
    // result_unwrap();
    println!("READ: {}", propagating_errors_questionmark_operator().expect("error propagating"));
}

fn result_match(){
let file = File::open("./src/chapter_9/error.txt");
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
    println!("{:?}", file);
}

fn result_unwrap() {
    //? unwrap result if not error, else panic!
    let file = File::open("./src/chapter_9/error.txt").unwrap(); 
}

fn result_expect() {
    //? unwraps with custom error message:   
    let file = File::open("./src/chapter_9/error.txt").expect("custom Error message when opening error.txt");
}
fn result_closures() {
    //? Result<T,E> accepts closures:
    let f = File::open("./src/chapter_9/error.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("./src/chapter_9/error.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f);
}

fn propagating_errors() -> Result<String, io::Error> {
    //? propagates Result<T,E> back to calling function
    //? called: println!("READ: {}", propagating_errors().expect("error propagating"));
    let file = File::open("./src/chapter_9/error.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut read = String::new();

    return match file.read_to_string(&mut read) {
        Ok(_) => Ok(read),
        Err(e) => Err(e),
    }; // no ; -> return
}

fn propagating_errors_questionmark_operator() -> Result<String, io::Error>{
    //? ? Operator returns error when unwrapping result fails.  
    //? ? is different from match, from function, 
    //? the error type received is converted into the error type defined 
    //? in the return type of the current function. This needs 'from' to be defined
    // let mut file = File::open("error.txt")?;
    // let mut read = String::new();
    // file.read_to_string(&mut read)?;
    // Ok(read)
    let mut read = String::new();
    File::open("./src/chapter_9/error.txt")?.read_to_string(&mut read)?;
    Ok(read)
}

fn read_username_from_file() -> Result<String, io::Error> {
    //? same as propagating_errors_questionmarkOperator
    //? questionmark error in main 
    // fn main() -> Result<(), Box<dyn Error>> {
    //     let f = File::open("hello.txt")?;

    //     Ok(())
    // }
    fs::read_to_string("./src/chapter_9/error.txt")
}