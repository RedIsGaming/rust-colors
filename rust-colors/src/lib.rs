use std::fmt;

#[derive(Debug)]
pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Default,
}

impl Colors {
    pub fn assign(&self) -> &str {
        match *self {
            Colors::Black => "[30m",
            Colors::Red => "[31m",
            Colors::Green => "[32m",
            Colors::Yellow => "[33m",
            Colors::Blue => "[34m",
            Colors::Purple => "[35m",
            Colors::Cyan => "[36m",
            Colors::White => "[37m",
            Colors::Default => "[0m",
        }
    }
}

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
