use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut measurement = 0;
    let mut count = 0;
    for line in f.lines() {
        let l = match line {
            Ok(v) => v,
            Err(_) => panic!("line is not valid"),
        };

        let number: u32 = l.parse().expect("line should be a number");
        if measurement != 0 && number > measurement {
            count += 1;
        }
        measurement = number;
    }

    println!("{}", count);
}
