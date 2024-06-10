macro_rules! color {
    ($($name:ident, $value:expr);*;) => {
        #[derive(Debug, PartialEq)]
        pub enum Colors {
            $($name,)*
        }

        impl Colors {
            pub fn assign(&self) -> &str {
                match *self {
                    $(Colors::$name => $value,)*
                }
            }
        }
    };
}

color! {
    Black, "[30m";
    Red, "[31m";
    Green, "[32m";
    Yellow, "[33m";
    Blue, "[34m";
    Purple, "[35m";
    Cyan, "[36m";
    White, "[37m";
    Default, "[0m";
}
