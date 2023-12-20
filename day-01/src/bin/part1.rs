use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = match File::open("./input.txt") {
        Err(_) => panic!("bad file"),
        Ok(file) => file,
    };

    let mut output: Vec<String> = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut val_array: Vec<char> = Vec::new();
        for c in line_str.chars() {
            if c.is_digit(10) {
                val_array.push(c);
                break;
            }
        }
        for c in line_str.chars().rev() {
            if c.is_digit(10) {
                val_array.push(c);
                break;
            }
        }
        output.push(val_array.iter().collect());
    }

    let mut sum: u32 = 0;
    for s in output.iter_mut() {
        sum += s.parse::<u32>().unwrap();
    }

    println!("final output: {}", sum);
}
