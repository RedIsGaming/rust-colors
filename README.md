# rust-colors
A library for parsing ANSI Strings to colors. Built in Rust, open source and easy to use. Underneath are some examples of how you could use the functions.

You can reach out to me on Discord: [RedIsGaming](https://discordapp.com/users/724341024415285319)<br/>

## Example color functions main program
Here is an example of how the main program can look:
```rust
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

```

## rust-colors options to choose from
```rust
Colors::Black,
Colors::Red,
Colors::Green,
Colors::Yellow,
Colors::Blue,
Colors::Purple,
Colors::Cyan,
Colors::White,
Colors::Default,

```

## Author
[RedIsGaming](https://github.com/RedIsGaming/)

## Language(s) and tools
- [Rust](https://www.rust-lang.org/tools/install)
- [VSCode](https://code.visualstudio.com/download)
    - rust-analyzer (extension)
    - Even Better TOML (extension)
    - Error Lens (extension)
    - crates (extension)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
    - clippy (feature)
    - fmt (feature)

## License
[MIT license](LICENSE)

## Status
<br/>
<span style="color: #0F9;">rust-colors last updated on 10-6-2024.</span><br/>
<ins style="font-size: 0px;">.</ins>

## Contact
[Discord](https://discordapp.com/users/724341024415285319)

## How to use this project?
### How to run?
First clone the rust-colors project to any location you desire.
```shell
git clone https://github.com/RedIsGaming/rust-colors.git
```

Then switch to that directory and run Cargo.
```shell
cd rust-colors
cargo run
```
