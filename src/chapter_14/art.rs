//! # Art
//!
//! A library for modeling artistic concepts.

// allows for the use of:
// use chapter_14::{
//     art::{mix, PrimaryColor, SecondaryColro},
//     crates,
// };
// instead of:
// use chapter_14::{
//      art: { kinds:: { PrimaryColor, SecondaryColor }, utils:: { mix } },
//      crates
// };

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Orange
    }
}
