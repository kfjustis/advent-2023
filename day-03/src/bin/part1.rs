use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
struct NumberIndex {
    line: i32,
    begin: i32,
    end: i32,
    value: i32,
    is_valid: bool
}

#[derive(Clone, Copy, Debug)]
struct SymbolIndex {
    line: i32,
    index: i32
}

fn main() {
    //let file = match File::open("./test.txt") { // 10 chars per line
    let file = match File::open("./input.txt") { // 140 chars per line
        Err(_) => panic!("bad file"),
        Ok(file) => file,
    };

    let mut line_number = 1;
    let mut symbol_indices: Vec<SymbolIndex> = Vec::new();
    let mut num_indices: Vec<NumberIndex> = Vec::new();

    let reader = BufReader::new(file);
    for itr in reader.lines() {
        let line = itr.unwrap();

        // convert line to characters and process each one.
        let mut c_index: i32 = 0;
        let mut num_vec: Vec<String> = Vec::new();
        let mut num_token: NumberIndex = NumberIndex{line: line_number,
            begin: -1,
            end: -1,
            value: -1,
            is_valid: false};
        let mut num_end: i32 = -1;
        for c in line.chars() {
            if !c.is_alphanumeric() {
                if c != '.' {
                    symbol_indices.push(SymbolIndex {line: line_number, index: c_index});
                }

                if num_end != -1 {
                    num_token.end = num_end;

                    // Convert the num_vec to a string and string to i32.
                    let num_string: String = num_vec.clone().into_iter().map(String::from).collect();
                    num_token.value = num_string.parse::<i32>().unwrap();

                    // Move the num bounds token into the num_indices for tracking;
                    num_indices.push(num_token);

                    // Reset the accumulators.
                    num_token.begin = -1;
                    num_token.end = -1;
                    num_token.value = -1;
                    num_token.is_valid = false;
                    num_vec.clear();
                    num_end = -1;
                }
            }
            else if c.is_digit(10) {
                // Save the first char of the number.
                if num_token.begin == -1 {
                    num_token.begin = c_index;
                }

                // Always mark the current number as the 'end' and push it.
                num_end = c_index;
                num_vec.push(c.to_string());

                if c_index == (line.char_indices().count() - 1) as i32 {
                    if num_end != -1 {
                        num_token.end = num_end;
    
                        // Convert the num_vec to a string and string to i32.
                        let num_string: String = num_vec.clone().into_iter().map(String::from).collect();
                        num_token.value = num_string.parse::<i32>().unwrap();
    
                        // Move the num bounds token into the num_indices for tracking;
                        num_indices.push(num_token);
    
                        // Reset the accumulators.
                        num_token.begin = -1;
                        num_token.end = -1;
                        num_token.value = -1;
                        num_token.is_valid = false;
                        num_vec.clear();
                        num_end = -1;
                    }
                }
            }
            c_index += 1;
        } // end chars
        line_number += 1;
    } // end lines

    //println!("star tokens:");
    //println!("{:?}", symbol_indices);
    //println!("num tokens:");
    //println!("{:?}", num_indices);

    //println!("\n---PROCESSING RESULTS---"); // 518930 is wrong.

    // process num tokens to see if they are valid.
    for n in num_indices.iter_mut() {
        let min_line = n.line - 1;
        let max_line = n.line + 1;

        for s in &symbol_indices {
            if s.line >= min_line && s.line <= max_line {
                if s.index >= n.begin - 1 && s.index <= n.end + 1 {
                    n.is_valid = true;
                    break;
                    //println!("{:?} because {:?}", n, s);
                } else {
                    //println!("{:?} because {:?}", n, s);
                }
            }
        }
    }

    let mut sum: i32 = 0;
    for &n in &num_indices {
        if n.is_valid {
            //if n.line < 10 {
            //    println!("valid: {:?}", n);
            //}
            sum += n.value;
        }
        //else if n.line < 10 {
        //    println!("invalid: {:?}", n);
        //}
    }
    println!("sum: {}", sum);
}
