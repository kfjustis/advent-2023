use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct GameInstance {
    id: u32,
    red: u32,
    blue: u32,
    green: u32,
}

const MAX_RED: u32 = 12;
const MAX_BLUE: u32 = 14;
const MAX_GREEN: u32 = 13;

fn main() {
    let file = match File::open("./input.txt") {
    //let file = match File::open("./test.txt") {
        Err(_) => panic!("bad file"),
        Ok(file) => file,
    };

    let mut line_number = 1;

    let mut id_sum: u32 = 0;

    let reader = BufReader::new(file);
    for itr in reader.lines() {
        let line = itr.unwrap();

        // Split line on tokens progressively to get the data we want.
        let data = line.split("; ");
        let mut game_id = 0;
        let mut game_possible: bool = true;
        for d in data {
            let game = d.split(": ");
            for g in game {
                if g.to_lowercase().contains("game") {
                    let parts = g.split(" ");
                    game_id = parts.last().unwrap().parse::<u32>().unwrap();
                }
                else {
                    let instance = process_game(game_id, g);
                    println!("\tinstance: {:?}", instance);
                    if !is_instance_valid(&instance) {
                        println!("\t\t...badGame!");
                        game_possible = false;
                    }
                }
            }
        }
        if game_possible {
            id_sum += game_id;
        }

        // Next itr.
        line_number += 1;
    }

    println!("output: {}", id_sum);
}

fn process_game(id: u32, game: &str) -> GameInstance {
    println!("processing game: {}", game);

    // Final container struct for the parsed game results.
    let mut result = GameInstance {id: id, red: 0, green: 0, blue: 0};

    // Split the game into each color draw.
    let items = game.split(", ");

    for i in items {
        let mut found_count: u32 = 0;
        let mut found_color: &str = "";

        let draw = i.split(" ");
        for d in draw {
            match d.parse::<u32>() {
                Ok(val) => {found_count = val},
                Err(e) => {
                    // ignore the error, we know this is the color
                    found_color = d;
                }
            }

            if found_color.contains("red") {
                result.red = found_count;
            } else if found_color.contains("green") {
                result.green = found_count;
            } else if found_color.contains("blue") {
                result.blue = found_count;
            }
        }
    }
    
    return result;
}

fn is_instance_valid(instance: &GameInstance) -> bool {
    if instance.red > MAX_RED ||
        instance.green > MAX_GREEN ||
        instance.blue > MAX_BLUE {
        return false;
    }
    return true;
}