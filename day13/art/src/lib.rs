//! # Art
//! 
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

mod kinds {
    /// Primary color according to the RYB color 
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// Secondary color according to the RYB color 
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        unimplemented!();
    } 
    
}