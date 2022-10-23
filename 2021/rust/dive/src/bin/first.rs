use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;
    for line in f.lines() {
        let text = line.expect("should be a line");
        let cmd: Vec<&str> = text.split(" ").collect();

        let value: u32 = cmd[1]
            .parse()
            .expect("value of the command should be a number");
        match cmd[0] {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("invalid command"),
        };
    }

    println!(
        "Depth is {} and horizontal is {}. Multiply: {}",
        depth,
        horizontal,
        depth * horizontal
    );
}
