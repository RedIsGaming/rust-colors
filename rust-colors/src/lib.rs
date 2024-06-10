//! # rust-colors
//! `rust-colors` is a library for parsing ANSI Strings to colors.
//!
//! ## Example
//! ```rust
//! use rust_colors::{Ansi, Color, Colors};
//!
//! fn my_function() {
//!     let colors = Ansi;
//!    
//!     println!(
//!         "The sky is {}, apples can be {}, do you like to touch {} grass?",
//!         colors.color("blue", Colors::Blue),
//!         colors.bold_color("red", Colors::Red),
//!         colors.underline_color("green", Colors::Green),
//!     );
//! }
//!
//! ```
//!
//! # Github
//! [Git repo RedIsGaming rust-colors](https://github.com/RedIsGaming/rust-colors)

pub use self::ansi::{Ansi, Color};
pub use self::colors::Colors;

mod colors;
mod ansi {
    use std::fmt;

    use crate::colors::Colors;

    #[derive(Debug)]
    pub struct Ansi;

    pub trait Color<T: fmt::Debug> {
        type Transform;

        fn color(&self, txt: T, color: Colors) -> Self::Transform;
        fn bold_color(&self, txt: T, bold_color: Colors) -> Self::Transform;
        fn underline_color(&self, txt: T, underline_color: Colors) -> Self::Transform;
    }

    impl<T: fmt::Debug> Color<T> for Ansi {
        type Transform = String;

        fn color(&self, txt: T, color: Colors) -> Self::Transform {
            format!(
                "\x1b{}{:?}\x1b{}",
                Colors::assign(&color),
                txt,
                Colors::assign(&Colors::Default)
            )
        }

        fn bold_color(&self, txt: T, bold_color: Colors) -> Self::Transform {
            format!(
                "\x1b[1;{}{:?}\x1b{}",
                Colors::assign(&bold_color).replace('[', ""),
                txt,
                Colors::assign(&Colors::Default)
            )
        }

        fn underline_color(&self, txt: T, underline_color: Colors) -> Self::Transform {
            format!(
                "\x1b[4;{}{:?}\x1b{}",
                Colors::assign(&underline_color).replace('[', ""),
                txt,
                Colors::assign(&Colors::Default)
            )
        }
    }
}
