use std::{env, process};
use std::fmt::Display;
use minigrep::Config;


fn main() {
    let args = env::args().collect();

    let config = Config::build(&args)
        .unwrap_or_else(|err| print_error_and_exit("Problem passing arguments: {}", err));

    if let Err(e) = minigrep::run(config) { print_error_and_exit("Application error", e) }
}

fn print_error_and_exit<T: Display>(message: &str, err: T) -> ! {
    eprintln!("{}: {}", message, err);
    process::exit(1);
}