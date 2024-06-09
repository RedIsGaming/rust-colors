use rust_colors::{Ansi, Color};

fn main() {
    let colors = Ansi;
    println!("{}, {}!", colors.red("Hello"), colors.cyan("World"));
}
