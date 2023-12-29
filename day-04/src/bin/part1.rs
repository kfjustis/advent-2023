use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //let file = match File::open("./test.txt") {
    let file = match File::open("./input.txt") {
        Err(_) => panic!("bad file"),
        Ok(file) => file,
    };

    let mut line_number = 1;
    let mut pts: u32 = 0;

    let reader = BufReader::new(file);
    for itr in reader.lines() {
        let line = itr.unwrap();

        let number_data = line.split(": ").last().unwrap();
        let win_have_data = number_data.split(" | ").collect::<Vec<&str>>();

        let mut win_data = win_have_data[0].split(" ").collect::<Vec<&str>>();
        win_data.retain(|&x| x != "");

        let mut have_data = win_have_data[1].split(" ").collect::<Vec<&str>>();
        have_data.retain(|&x| x != "");

        let mut line_pts = 0;

        for n in have_data {
            if win_data.contains(&n) {
                //println!("line {} match! -> {}", line_number, n);
                if line_pts == 0 {
                    line_pts += 1;
                } else {
                    line_pts *= 2;
                }
            }
        }

        pts += line_pts;

        line_number += 1;
    }

    println!("...final pts: {}", pts);
}
