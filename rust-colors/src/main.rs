use rust_colors::{Ansi, Color, Colors};

fn main() {
    let colors = Ansi;

    println!(
        "The sky is {}, apples can be {}, do you like to touch {} grass?",
        colors.color("blue", Colors::Blue),
        colors.bold_color("red", Colors::Red),
        colors.underline_color("green", Colors::Green),
    );
}
