use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct CardData {
    id: u32,
    num_matches: u32
}

fn main() {
    //let file = match File::open("./test.txt") {
    let file = match File::open("./input.txt") {
        Err(_) => panic!("bad file"),
        Ok(file) => file,
    };

    let mut line_number = 1;
    let mut card_set: Vec<CardData> = Vec::new();
    let mut pipe: Vec<CardData> = Vec::new();

    let reader = BufReader::new(file);
    for itr in reader.lines() {
        let line = itr.unwrap();

        let number_data = line.split(": ").last().unwrap();
        let win_have_data = number_data.split(" | ").collect::<Vec<&str>>();

        let mut win_data = win_have_data[0].split(" ").collect::<Vec<&str>>();
        win_data.retain(|&x| x != "");

        let mut have_data = win_have_data[1].split(" ").collect::<Vec<&str>>();
        have_data.retain(|&x| x != "");

        let mut line_matches = 0;

        for n in have_data {
            if win_data.contains(&n) {
                line_matches += 1;
            }
        }

        card_set.push(CardData{id: line_number, num_matches: line_matches});
        card_set.dedup();

        line_number += 1;
    }

    // Build the card pipe from the generated hash vec.
    for c in &card_set {
        add_cards_recursive(c.clone(), &card_set, &mut pipe);
    }

    println!("...total cards: {}", pipe.len());
}

fn add_cards_recursive(card: CardData, card_set: &Vec<CardData>, pipe: &mut Vec<CardData>) {
    // push the original to the pipe.
    pipe.push(card.clone());
    //println!("pushed card {}", card.id);

    if card.num_matches == 0 {
        return;
    }

    // recursive push all of the copies.
    let min_id: usize = card.id as usize;
    let max_id: usize = (card.id + card.num_matches) as usize;
    for i in min_id..max_id {
        add_cards_recursive(card_set[i].clone(), card_set, pipe);
    }
}
