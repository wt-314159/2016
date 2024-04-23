#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = String::from("11101000110010100");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    // Part 1
    //let disk_size = 272;
    // Part 2
    let disk_size = 35651584;
    let mut data = input;
    while data.len() < disk_size {
        data = iterate_dragon_curve(&data);
    }

    println!("Disk filled, now generating checksum");
    //println!("Disk data: {}", data);
    let disk_data = &data[0..disk_size];
    let mut checksum = generate_checksum(disk_data);
    let mut check_length = checksum.len();
    while check_length % 2 == 0 {
        checksum = generate_checksum(&checksum);
        check_length = checksum.len();
        println!("Checksum length: {check_length}");
    }

    println!("Checksum: {}", checksum);
}

fn iterate_dragon_curve(input: &str) -> String {
    let b = input.chars().rev().map(|c| 
        { 
            match c { 
                '0' => '1',
                '1' => '0',
                other => panic!("Unexpected character: '{}'", other)
            }
        })
        .collect::<String>();
    format!("{}0{}", input, b)
}

fn generate_checksum(input: &str) -> String {
    let mut prev_char = '-';
    let mut odd = false;
    let mut chars: Vec<char> = Vec::new();
    for c in input.chars() {
        if odd {
            if c == prev_char {
                chars.push('1');
            }
            else {
                chars.push('0');
            }
        }
        else {
            prev_char = c;
        }
        odd = !odd;
    }
    chars.iter().collect::<String>()
}