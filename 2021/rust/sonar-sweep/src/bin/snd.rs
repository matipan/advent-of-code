use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let buf = BufReader::new(f);

    let mut measurements: Vec<u32> = Vec::new();
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut count: u32 = 0;
    for line in buf.lines() {
        // we push the measurement to the vec
        measurements.push(
            line.expect("should be a line")
                .parse()
                .expect("line should be a number"),
        );
        // increment the end pointer by 1
        end += 1;

        // Check if we have enough elements, if not we continue
        if end - start <= 3 {
            continue;
        }

        // if not, we do the magic. Grab the two subslices, sum and compare
        let first: u32 = measurements[start..(start + 3)].iter().sum();
        let second: u32 = measurements[end - 3..end].iter().sum();

        if second > first {
            count += 1;
        }

        // move the start pointer by one now that we've done the calculations
        start += 1;
    }

    println!("{}", count);
}
