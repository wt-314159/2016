#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Md5, Digest};
// use priority_queue::PriorityQueue;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    
    let scrambled = scramble_password("abcdefgh", &input);
    println!("Completed, scrambled password: {scrambled}");

    let unscrambled = unscramble_password("fbgdceah", &input);
    println!("Unscrambled {}", unscrambled);
}

fn unscramble_password(scrambled: &str, input: &str) -> String {
    let mut password = String::from(scrambled);
    for line in input.split("\n").collect::<Vec<&str>>().iter().rev() {
        password = parse_line_and_unscramble(&password, line);
        
        //println!("password: {}", password);
    }
    password
}

fn parse_line_and_unscramble(password: &str, line: &str) -> String {
    let params = line.split_whitespace().collect::<Vec<&str>>();
    match params[0] {
        // swaps are reversible, so do exactly as it says to undo
        "swap" => match params[1] {
            "position" => swap_positions(&password, params[2].parse::<usize>().unwrap(), params[5].parse::<usize>().unwrap()),
            "letter" => swap_characters(&password, params[2], params[5]),
            other => panic!("Don't recognise 'swap {other}'")
        },
        "rotate" => match params[1] {
            "left" => rotate_right(&password, params[2].parse::<usize>().unwrap()),
            "right" => rotate_left(&password, params[2].parse::<usize>().unwrap()),
            "based" => unrotate_based_on(&password, params[6]),
            other => panic!("Don't recognise 'rotate {other}'")
        },
        "reverse" => reverse(&password, params[2].parse::<usize>().unwrap(), params[4].parse::<usize>().unwrap()),
        "move" => move_to(&password, params[5].parse::<usize>().unwrap(), params[2].parse::<usize>().unwrap()),
        other => panic!("Don't recognise '{other}'")
    }
}

fn scramble_password(password: &str, input: &str) -> String {
    let mut scrambled = String::from(password);
    for line in input.split("\n") {
        scrambled = parse_line_and_apply(&scrambled, line);

        //println!("Password: {scrambled}");
    }
    scrambled
}

fn parse_line_and_apply(password: &str, line: &str) -> String {
    let params = line.split_whitespace().collect::<Vec<&str>>();
    match params[0] {
        "swap" => match params[1] {
            "position" => swap_positions(&password, params[2].parse::<usize>().unwrap(), params[5].parse::<usize>().unwrap()),
            "letter" => swap_characters(&password, params[2], params[5]),
            other => panic!("Don't recognise 'swap {other}'")
        },
        "rotate" => match params[1] {
            "left" => rotate_left(&password, params[2].parse::<usize>().unwrap()),
            "right" => rotate_right(&password, params[2].parse::<usize>().unwrap()),
            "based" => rotate_based_on(&password, params[6]),
            other => panic!("Don't recognise 'rotate {other}'")
        },
        "reverse" => reverse(&password, params[2].parse::<usize>().unwrap(), params[4].parse::<usize>().unwrap()),
        "move" => move_to(&password, params[2].parse::<usize>().unwrap(), params[5].parse::<usize>().unwrap()),
        other => panic!("Don't recognise '{other}'")
    }
}

fn swap_positions(input: &str, x: usize, y: usize) -> String {
    let mut owned = input.to_owned();
    let x_str = &input[x..x+1];
    let y_str = &input[y..y+1];
    owned.replace_range(x..x+1, y_str);
    owned.replace_range(y..y+1, x_str);
    owned
}

fn swap_characters(input: &str, x: &str, y: &str) -> String {
    input.replace(x, "_")
    .replace(y, x)
    .replace('_', y)
}

fn rotate_right(input: &str, mut x: usize) -> String {
    while x > input.len() {
        x -= input.len();
    }
    let tail_start = input.len() - x;
    let tail = &input[tail_start..];
    let start = &input[0..tail_start];
    tail.to_owned() + start
}

fn rotate_left(input: &str, mut x: usize) -> String {
    while x > input.len() {
        x -= input.len();
    }
    let start = &input[..x];
    let tail = &input[x..];
    tail.to_owned() + start
}

fn rotate_based_on(input: &str, x: &str) -> String {
    let index = input.find(x).unwrap();
    if index >= 4 {
        rotate_right(input, index + 2)
    }
    else {
        rotate_right(input, index + 1)
    }
}

// N.B. Works for 8 characters, not for 5
fn unrotate_based_on(input: &str, x: &str) -> String {
    let mut index = input.find(x).unwrap();
    if index == 0 {
        index = input.len();
    }
    // we always rotate right by at least 1
    index -= 1;
    let mut left = 1;
    // index ends up being 
    if index % 2 != 0 {
        index -= 1;
        left += 1;
        index += input.len();
    }
    left += index / 2;
    rotate_left(input, left)
}

fn reverse(input: &str, x: usize, y: usize) -> String {
    let to_reverse = &input[x..=y];
    input[0..x].to_owned() + &to_reverse.chars().rev().collect::<String>() + &input[y + 1..]
}

fn move_to(input: &str, x: usize, y: usize) -> String {
    let mut owned = input.to_owned();
    let to_move = owned.remove(x);
    owned.insert(y, to_move);
    owned
}