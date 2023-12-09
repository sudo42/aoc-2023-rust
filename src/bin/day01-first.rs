use std::env;
use std::io::{stdin, BufRead, BufReader};
use std::fs::File;

const ASCII_ZERO: u32 = '0' as u32;
const SUB_PER_LINE: u32 = ASCII_ZERO * 10 + ASCII_ZERO;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = match &args[..] {
        [] | [_]    => None,
        [_, path]   => if path == "-" { None } else { Some(path) },
        _           => panic!("eeek! too many args … qq"),
    };
    let reader : Box<dyn BufRead> = match path {
        None        => Box::new(BufReader::new(stdin().lock())),
        Some(path)  => Box::new(BufReader::new(File::open(path).unwrap())),
    };
    let sum = reader.lines().map(|line| {
        let line = line.unwrap();
        match (
            line.chars().find(|c: &char| c.is_ascii_digit()),
            line.chars().rfind(|c: &char| c.is_ascii_digit())
        ) {
            //(Some(first), Some(last)) => ((first as u32) - ASCII_ZERO) * 10 + (last as u32) - ASCII_ZERO,
            (Some(first), Some(last)) => (first as u32) * 10 + (last as u32) - SUB_PER_LINE,
            (_,_) => 0, //panic!("not enough numbers in that line: {}", line),
        }
    }).fold(0u32, |accumulator, entry| accumulator + entry);
    eprint!("\ncalibration value: ");
    println!("{}", sum);
}
