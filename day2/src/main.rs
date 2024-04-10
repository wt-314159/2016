#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let keypad = [['*', '*', '1', '*', '*'], 
                                  ['*', '2', '3', '4', '*'],
                                  ['5', '6', '7', '8', '9'], 
                                  ['*', 'A', 'B', 'C', '*'],
                                  ['*', '*', 'D', '*', '*']];

    let mut row: usize = 2;
    let mut col: usize = 0;

    #[allow(unused_variables)]
    for line in input.split("\n") {
        for c in line.chars() {
            // For Part 1, change get_min_...() to 0, and get_max_...() to 3 
            match c {
                'L' => if col > get_min_row_or_col(row) {
                    col -= 1;
                },
                'R' => if col < get_max_row_or_col(row) {
                    col += 1;
                },
                'U' => if row > get_min_row_or_col(col) {
                    row -= 1;
                },
                'D' => if row < get_max_row_or_col(col) {
                    row += 1;
                }
                other => panic!("Can't handle {}", other)
            }
        }
        // For Part 1, use first line
        //println!("Press button {}", get_num_from_row_and_col(row, col));
        println!("Press button {}", get_char_from_row_and_col(row, col, &keypad));
    }
}

fn get_num_from_row_and_col(row: usize, col: usize) -> usize {
    1 + row * 3 + col
}

fn get_char_from_row_and_col(row: usize, col: usize, keypad: &[[char; 5]; 5]) -> char {
    keypad[row][col]
}

fn get_min_row_or_col(row_or_col: usize) -> usize {
    match row_or_col {
        0 => 2,
        1 => 1,
        2 => 0,
        3 => 1,
        4 => 2,
        other => panic!("min panic: {}", other)
    }
}

fn get_max_row_or_col(row_or_col: usize) -> usize {
    match row_or_col {
        0 => 2,
        1 => 3,
        2 => 4,
        3 => 3,
        4 => 2,
        other => panic!("max row panic: {}", other)
    }
}
