// mod median;
mod chapter_1_8;
mod chapter_9;
mod chapter_10;
use chapter_1_8::{departments, median, pig_latin, temperature};
use chapter_9::{error, guessing_game};
use chapter_10::{generic_types};

fn main() {
    generic_types::new();
    // guessing_game::start();
    // error::error(); 
    // departments::loop_init();
    // temperature::sample_temperature();
    // temperature::get_user_input();
    // println!("{}", median::get_mean(&[0, 5])); // -> 2.5?
    // println!("{}", median::get_median(&[0, 5, 10])); // -> 5
    // println!("{:?}", median::get_mode(&[0, 1, 1, 2, 3, 4, 4, 4])); // -> 4
    // println!("{:?}", pig_latin::convert_to_piglatin("first".to_string()));
}
