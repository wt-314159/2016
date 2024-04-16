#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = String::from("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN");
    //println!("{:?}", input);
    println!("Input length: {}", input.len());

    //let version1_length = get_decompressed_length(&input);
    //println!("Decompressed length: {}", version1_length);
    let version2_length = get_version2_length(&input);
    println!("version 2 length {}", version2_length);
}

fn get_version2_length(input: &str) -> usize {
    let mut index: usize = 0;
    let mut length_counter: usize = 0;
    let input_chars: Vec<char> = input.chars().collect();

    while index < input.len() {
        let char = input_chars[index];
        if char == '(' {
            let close_bracket_offset = input_chars.iter().skip(index).position(|x| *x == ')').unwrap();
            // markers don't end up in decompressed data, so don't count towards length
            //length_counter += close_bracket_offset + 1;
            let close_bracket = index + close_bracket_offset;
            let marker = &input[index + 1..close_bracket];

            let mut params = marker.split("x");
            let length = params.next().unwrap().parse::<usize>().unwrap();
            let repeats = params.next().unwrap().parse::<usize>().unwrap();

            let to_repeat = &input[close_bracket + 1..close_bracket + 1 + length];
            let decompressed_repeat_length = get_version2_length(to_repeat);
            let multiplied_length = decompressed_repeat_length * repeats;
            length_counter += multiplied_length;
            index = close_bracket + length + 1;
        }
        else if let Some(open_bracket_offset) = input_chars.iter().skip(index).position(|x| *x == '(') {
            length_counter += open_bracket_offset;
            index += open_bracket_offset;
        }
        else {
            let remaining = input.len() - index;
            length_counter += remaining;
            break;
        }
    }

    length_counter
}

#[allow(dead_code)]
fn get_decompressed_length(input: &str) -> usize {
    let mut index = 0;
    let mut decompressed: Vec<&str> = Vec::new();
    let input_chars: Vec<char> = input.chars().collect();
    while index < input.len() {
        let char = input_chars[index];
        if char == '(' {
            let close_bracket = input_chars.iter().skip(index).position(|x| *x == ')').unwrap() + index;
            let marker = &input[index + 1..close_bracket];
            let params: Vec<&str> = marker.split("x").collect();
            let length = params[0].parse::<usize>().unwrap();
            let repeats = params[1].parse::<usize>().unwrap();

            let to_repeat = &input[close_bracket + 1..close_bracket + 1 + length];
            for _ in 0..repeats {
                decompressed.push(to_repeat);
            }
            index = close_bracket + length + 1;
        }
        else if let Some(open_bracket_offset) = input_chars.iter().skip(index).position(|x| *x == '(') {
            let open_bracket = index + open_bracket_offset;
            let plain_text = &input[index..open_bracket];
            decompressed.push(plain_text);
            index = open_bracket;
        }
        else {
            // No more brackets, push rest to vec
            let plain_text = &input[index..];
            decompressed.push(plain_text);
            index = input.len();
        }
    }

    let output = decompressed.concat().replace(" ", "").replace("\n", "");
    output.len()
}
