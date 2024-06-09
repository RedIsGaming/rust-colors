use rust_colors::{Ansi, Color};

fn main() {
    let colors = Ansi;
    
    println!(
        "Roses are {}, the sky is {}, I like {} vegetables, but the color {} is even better!",
        colors.red("red"),
        colors.blue("blue"),
        colors.green("green"),
        colors.black("black"),
    );
}
