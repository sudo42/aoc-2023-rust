use std::env;
use std::io::{stdin, BufReader};
use std::fs::File;

pub use std::io::BufRead;

#[cfg(WITH_DEBUG_SECTIONS="true")]
macro_rules! debug_only {
    (default; $expression:expr) => { $expression; };
    ($expression:expr) => { $expression; };
}
#[cfg(WITH_DEBUG_SECTIONS="false")]
macro_rules! debug_only {
    (default; $expression:expr) => {};
    ($expression:expr) => {};
}
#[cfg(WITH_DEBUG_SECTIONS="default")]
macro_rules! debug_only {
    (default; $expression:expr) => { $expression; };
    ($expression:expr) => {};
}
pub(crate) use debug_only;

pub fn input_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [] | [_]    => None,
        [_, path]   => if path == "-" { None } else { Some(path.clone()) },
        _           => panic!("eeek! too many args … qq"),
    }
}

pub fn file_or_stdin_reader (path: Option<String>) -> Box<dyn BufRead> {
    let reader : Box<dyn BufRead> = match path {
        None        => Box::new(BufReader::new(stdin().lock())),
        Some(path)  => Box::new(BufReader::new(File::open(path).unwrap())),
    };
    return reader;
}

pub fn default_input_reader() -> Box<dyn BufRead> {
    file_or_stdin_reader(input_path_from_args())
}
