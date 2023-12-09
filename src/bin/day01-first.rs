//use std::io::BufRead;

mod common;
use crate::common::*;

const ASCII_ZERO: u32 = '0' as u32;
const SUB_PER_LINE: u32 = ASCII_ZERO * 10 + ASCII_ZERO;

fn main() {
    let reader = default_input_reader();
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
