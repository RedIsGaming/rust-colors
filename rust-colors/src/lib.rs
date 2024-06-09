//! # rust-colors
//!
//! `rust-colors` is a library for parsing ANSI Strings to colors. 
//! 
//! ## Example
//! ```
//! use rust_colors::{Ansi, Color};
//! 
//! fn my_function() {
//!     let colors = Ansi;
//!    
//!     println!(
//!         "Roses are {}, the sky is {}, I like {} vegetables, but the color {} is even better!",
//!         colors.red("red"),
//!         colors.blue("blue"),
//!         colors.green("green"),
//!         colors.black("black"),
//!     );
//! }
//! 
//! ```
//! 
//! # Github
//! [Git repo RedIsGaming rust-colors](https://github.com/RedIsGaming/rust-colors)

pub use self::ansi_color::{Ansi, Color};
mod colors;

pub mod ansi_color {
    use std::fmt;
    use crate::colors::Colors;

    #[derive(Debug)]
    pub struct Ansi;

    pub trait Color<T: fmt::Debug> {
        type Transform;

        fn black(&self, txt: T) -> Self::Transform;
        fn red(&self, txt: T) -> Self::Transform;
        fn green(&self, txt: T) -> Self::Transform;
        fn yellow(&self, txt: T) -> Self::Transform;
        fn blue(&self, txt: T) -> Self::Transform;
        fn purple(&self, txt: T) -> Self::Transform;
        fn cyan(&self, txt: T) -> Self::Transform;
        fn white(&self, txt: T) -> Self::Transform;
    }

    impl<T: fmt::Debug> Color<T> for Ansi {
        type Transform = String;

        fn black(&self, txt: T) -> Self::Transform {
            format!("\x1b{}{:?}\x1b{}", Colors::assign(&Colors::Black), txt, Colors::assign(&Colors::Default))
        }

        fn red(&self, txt: T) -> Self::Transform {
            format!("\x1b{}{:?}\x1b{}", Colors::assign(&Colors::Red), txt, Colors::assign(&Colors::Default))
        }

        fn green(&self, txt: T) -> Self::Transform {
            format!("\x1b{}{:?}\x1b{}", Colors::assign(&Colors::Green), txt, Colors::assign(&Colors::Default))
        }

        fn yellow(&self, txt: T) -> Self::Transform {
            format!("\x1b{}{:?}\x1b{}", Colors::assign(&Colors::Yellow), txt, Colors::assign(&Colors::Default))
        }

        fn blue(&self, txt: T) -> Self::Transform {
            format!("\x1b{}{:?}\x1b{}", Colors::assign(&Colors::Blue), txt, Colors::assign(&Colors::Default))
        }

        fn purple(&self, txt: T) -> Self::Transform {
            format!("\x1b{}{:?}\x1b{}", Colors::assign(&Colors::Purple), txt, Colors::assign(&Colors::Default))
        }

        fn cyan(&self, txt: T) -> Self::Transform {
            format!("\x1b{}{:?}\x1b{}", Colors::assign(&Colors::Cyan), txt, Colors::assign(&Colors::Default))
        }

        fn white(&self, txt: T) -> Self::Transform {
            format!("\x1b{}{:?}\x1b{}", Colors::assign(&Colors::White), txt, Colors::assign(&Colors::Default))
        }
    }
}
