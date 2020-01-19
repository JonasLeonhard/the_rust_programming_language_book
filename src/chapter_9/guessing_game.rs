use::std::{io};
use::rand::Rng;
#[derive(Debug)]
struct Guess {
    value: i32,
}
impl Guess {
    fn new(value: i32) -> Guess {
        if value < -100 || value > 100 {
            panic!("value must be between -100 and 100, got {}", value)
        }

        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

pub fn start() {
    loop {
        println!("Enter a guess between 1 and 100. Type quit to exit()");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("InputError:"); 

        if buffer.trim() == "quit"{
            break;
        }

        let parsed : i32 = buffer.trim().parse().unwrap();
        let guess : Guess = Guess::new(parsed);
        let random : i32 = rand::thread_rng().gen_range(0, 100);
        if random == guess.value{ 
            println!("you guessed right!");
            break;
        }
        println!("your guess was:: {:?}, right was: {:?}", guess, random);
    }
}