use std::env;
use std::io::{stdin, BufRead, BufReader};
use std::fs::File;


/*
eight
five
four
nine
one
seven
six
three
two
*/

const ASCII_ZERO: u32 = '0' as u32;

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

        let mut first: Option<u64> = None;
        let mut last: Option<u64> = None;

        let mut slice = line.as_str();
        while !slice.is_empty() {
            let c0 = slice.chars().next().unwrap();
            slice = &slice[1..];
            //let c0 = slice.chars().next().unwrap();
            let num = match c0 {
                c0 if c0.is_ascii_digit() => Some((c0 as u32) - ASCII_ZERO),
                'o' if slice.starts_with("ne")     => { slice = &slice[2..]; Some(1) },
                't' if slice.starts_with("wo")     => { slice = &slice[2..]; Some(2) },
                't' if slice.starts_with("hree")   => { slice = &slice[4..]; Some(3) },
                'f' if slice.starts_with("our")    => { slice = &slice[3..]; Some(4) },
                'f' if slice.starts_with("ive")    => { slice = &slice[3..]; Some(5) },
                's' if slice.starts_with("ix")     => { slice = &slice[2..]; Some(6) },
                's' if slice.starts_with("even")   => { slice = &slice[4..]; Some(7) },
                'e' if slice.starts_with("ight")   => { slice = &slice[4..]; Some(8) },
                'n' if slice.starts_with("ine")    => { slice = &slice[3..]; Some(9) },
                _ => None,
            };
            if let Some(num) = num {
                let _ = last.insert(num.into());
                first.get_or_insert(num.into());
            }
        }
        let line_num = match (first, last) {
            (Some(first), Some(last)) => first * 10 + last,
            (_,_) => 0, //panic!("not enough numbers in that line: {}", line),
        };
        eprintln!("-> {:?} {:?} -> {} for line: {}", first, last, line_num, line);
        line_num
    }).fold(0u64, |accumulator, entry| accumulator + entry);
    eprint!("\ncalibration value: ");
    println!("{}", sum);
}
