// mod median;
mod chapter_10;
mod chapter_11;
mod chapter_12;
mod chapter_13;
mod chapter_14;
mod chapter_1_8;
mod chapter_9;

use chapter_10::generic_types_traits_lifetimes;
use chapter_11::tests;
use chapter_12::basicgrep;
use chapter_13::{closures, iterators};
use chapter_14::crates;
use chapter_1_8::{departments, median, pig_latin, temperature};
use chapter_9::{error, guessing_game};

use std::{io, process};

fn main() {
    loop {
        println!("Loaded: THE_RUST_PROGAMMING_BOOK. \nChoose a Chapter: type a [number]");
        println!("1. pig_latin_convert");
        println!("2. get medians");
        println!("3. temperature");
        println!("4. Add to departments hashmap");
        println!("5. Error Handling");
        println!("6. Guessing Game");
        println!("7. Generic types, traits and lifetimes");
        println!("8. Unit Test & Implementation Tests");
        println!("9. Basic grep implementation");
        println!("10. Functional Language Features: Iterators and Closures");
        println!("11. Crates");
        println!("quit. Exit()\n");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("InputError:");

        let input = buffer.trim();
        println!("\nChoose {}:\n ", input);
        match input {
            "1" => {
                println!("{:?}", pig_latin::convert_to_piglatin("first".to_string()));
            }
            "2" => {
                println!("{:?}", median::get_mode(&[0, 1, 1, 2, 3, 4, 4, 4]));
                println!("{}", median::get_median(&[0, 5, 10]));
                println!("{}", median::get_mean(&[0, 5]));
            }
            "3" => {
                temperature::sample_temperature();
                temperature::get_user_input();
            }
            "4" => {
                departments::loop_init();
            }
            "5" => {
                error::error();
            }
            "6" => {
                guessing_game::start();
            }
            "7" => {
                generic_types_traits_lifetimes::new();
            }
            "8" => {
                tests::new();
            }
            "9" => {
                basicgrep::new();
            }
            "10" => {
                closures::new();
                iterators::new();
            }
            "11" => crates::new(),
            "quit" => {
                process::exit(0);
            }
            _ => {}
        }

        println!("\n Press any key to run again\n");
        io::stdin().read_line(&mut buffer).expect("InputError:");
    }
}
