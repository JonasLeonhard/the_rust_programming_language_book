//! some description comment for this file, instead of the following comment

/// this is a doc comment for crate documentation markdown via ///
/// open the docs with cargo doc --open
/// # Headline
/// ```
/// crate::new()
/// ```
pub fn new() {
    println!("crates.rs called")
}

/// Adds one to the number given. This comment is also run as a Test via cargo test
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
