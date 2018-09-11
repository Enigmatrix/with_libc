#![macro_use]

macro_rules! info {
    ($($args:tt)*) => {
        println!("[{}] {}",
                 ansi_term::Color::Blue.bold().paint(format!("*")),
                 format!($($args)*))
    };
}

macro_rules! warn {
    ($($args:tt)*) => {
        println!("[{}] {}",
                 ansi_term::Color::Red.bold().paint(format!("!")),
                 format!($($args)*))
    };
}
