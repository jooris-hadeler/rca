#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{} {}", "info:".bold().blue(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "warn:".bold().yellow(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "error:".bold().red(), format!($($arg)*))
    };
}
