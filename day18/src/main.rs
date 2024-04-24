#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Md5, Digest};
// use priority_queue::PriorityQueue;

const ROW_SIZE: usize = 100;
//const ROW_SIZE: usize = 5;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = String::from("..^^.");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut first_row = [false; ROW_SIZE];
    for (i, c) in input.chars().enumerate() {
        first_row[i] = match c {
            '.' => false,
            '^' => true,
            other => panic!("Cannot process {other}")
        }
    }

    let mut next_row = first_row;
    //print_row(next_row);
    let mut num_safe = num_safe_tiles(first_row);
    // For Part 1, change this to 39
    for _ in 0..399999 {
        next_row = generate_next_row(next_row);
        num_safe += num_safe_tiles(next_row);
        //print_row(next_row);
    }

    println!("Total safe tiles: {}", num_safe);
}

fn generate_next_row(prev_row: [bool; ROW_SIZE]) -> [bool; ROW_SIZE] {
    let mut next_row = [false; ROW_SIZE];
    for i in 1..ROW_SIZE - 1 {
        next_row[i] = is_trap(prev_row[i - 1], prev_row[i], prev_row[i + 1]);
    }
    next_row[0] = is_trap(false, prev_row[0], prev_row[1]);
    next_row[ROW_SIZE - 1] = is_trap(prev_row[ROW_SIZE - 2], prev_row[ROW_SIZE - 1], false);
    next_row
}

fn is_trap(left: bool, center: bool, right: bool) -> bool {
    left && center && !right ||
    !left && center && right ||
    left && !center && !right ||
    !left && !center && right
}

fn num_safe_tiles(row: [bool; ROW_SIZE]) -> usize {
    row.iter().filter(|x| !**x).count()
}

fn print_row(row: [bool; ROW_SIZE]) {
    for t in row {
        match t {
            true => print!("^"),
            false => print!(".")
        }
    }
    println!();
}