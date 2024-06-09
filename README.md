# rust-colors
A library for parsing ANSI Strings to colors. Built in Rust, open source and easy to use. Underneath are some examples of how you could use the functions.

> You can reach out to me on Discord: [RedIsGaming](https://discordapp.com/users/724341024415285319)<br/>

## Example color functions main program
Here is an example of how the main program can look:
```rs
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

```

> Roses are <span style="color: rgb(255, 85, 0)">"red"</span>, the sky is <span style="color: rgb(0, 140, 255)">"blue"</span>, I like <span style="color: rgb(0, 200, 0)">"green"</span> vegetables, but the color <span style="color: rgb(130, 130, 130)">"black"</span> is even better!

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
> <br/>
> <span style="color: #0F9;">rust-colors last updated on 9-6-2024.</span><br/>
> <ins style="font-size: 0px;">.</ins>

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

## Color functions rust-colors(ANSI)
Here is a quick overview of functions that are applied for this project:

### Struct
```rs
#[derive(Debug)]
pub struct Ansi;
```

### Trait
```rs
pub trait Color<T: fmt::Debug> {
    type Transform;
}
```

### Trait functions
```rs
//Required functions:
fn black(&self, txt: T) -> Self::Transform;
fn red(&self, txt: T) -> Self::Transform;
fn green(&self, txt: T) -> Self::Transform;
fn yellow(&self, txt: T) -> Self::Transform;
fn blue(&self, txt: T) -> Self::Transform;
fn purple(&self, txt: T) -> Self::Transform;
fn cyan(&self, txt: T) -> Self::Transform;
fn white(&self, txt: T) -> Self::Transform;
```
