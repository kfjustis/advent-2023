use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct MyToken {
    index: u32,
    data: String,
}

fn main() {
    let file = match File::open("./input.txt") {
        Err(_) => panic!("bad file"),
        Ok(file) => file,
    };

    // Define value literals.
    const VALUES: [&'static str; 9] =
    [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let mut line_num = 1;
    let mut output: Vec<String> = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut line_tokens: Vec<MyToken> = Vec::new();

        // When a value string is found in the line, add its token with index.
        // Make sure all occurrences for each value are discovered.
        // Example: oneonetwotwothreetwo -> 12, not 13.
        let mut val_idx: u32 = 0;
        for val in VALUES.iter() {
            let val_literal = val_idx + 1;
            let occurrences: Vec<_> = line_str.match_indices(val).collect();
            for o in occurrences {
                line_tokens.push(MyToken {index: o.0 as u32, data: val_literal.to_string()});
            }
            val_idx += 1;
        }

        // Next, find the actual digits and add them as tokens with their index.
        let mut c_idx = 0;
        for c in line_str.chars() {
            if c.is_digit(10) {
                line_tokens.push(MyToken {index: c_idx as u32, data: c.to_string()});
            }
            c_idx += 1;
        }

        // Sort the tokens by their index for the line.
        line_tokens.sort_by_key(|d| d.index);

        // Combine the first and last token values to make one number.
        let left = &line_tokens.first().unwrap().data;
        let right = &line_tokens.last().unwrap().data;
        let number = format!("{left}{right}");

        println!("line {}: {}", line_num, number);
        output.push(number);

        line_num += 1;
    }

    let mut sum: u32 = 0;
    for s in output.iter_mut() {
        sum += s.parse::<u32>().unwrap();
    }

    println!("final output: {}", sum);
}
