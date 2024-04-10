#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut characters = [['_'; 624]; 8];
    for (r, line) in input.split("\n").enumerate() {
        for (i, c) in line.char_indices() {
            characters[i][r] = c;
        }
    }

    let mut message = ['_'; 8];
    for i in 0..8 {
        let most_common_char = find_most_common_char(characters[i]);
        message[i] = most_common_char;
    }

    let mut min_message = ['-'; 8];
    for i in 0..8 {
        let least_common_char = find_least_common_char(characters[i]);
        min_message[i] = least_common_char;
    }

    let message_string: String = message.iter().collect();
    let min_message_string: String = min_message.iter().collect();
    println!("Most common message is {}", message_string);
    println!("Least commmon message is {}", min_message_string);
}

fn find_most_common_char(chars: [char; 624]) -> char {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for c in chars {
        if char_count.contains_key(&c) {
            *char_count.get_mut(&c).unwrap() += 1;
        }
        else {
            char_count.insert(c, 1);
        }
    }
    
    let max =char_count.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    max.0.clone()
}

fn find_least_common_char(chars: [char; 624]) -> char {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for c in chars {
        if char_count.contains_key(&c) {
            *char_count.get_mut(&c).unwrap() += 1;
        }
        else {
            char_count.insert(c, 1);
        }
    }
    
    let max =char_count.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    max.0.clone()
}
