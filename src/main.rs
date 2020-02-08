// mod median;
mod chapter_10;
mod chapter_11;
mod chapter_12;
mod chapter_1_8;
mod chapter_9;

use chapter_10::generic_types_traits_lifetimes;
use chapter_11::tests;
use chapter_12::basicgrep;
use chapter_1_8::{departments, median, pig_latin, temperature};
use chapter_9::{error, guessing_game};

fn main() {
    basicgrep::new();
    // tests::new();
    // generic_types_traits_lifetimes::new();
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
