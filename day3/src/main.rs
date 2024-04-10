#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut num_poss = 0;
    for line in input.split("\n") {
        let sides: Vec<&str> = line.split_whitespace().collect();
        let mut side_lengths: Vec<usize> = sides.iter().map(|x| x.parse::<usize>().unwrap()).collect();
        side_lengths.sort();

        let two_sides = side_lengths[0] + side_lengths[1];
        if two_sides > side_lengths[2] {
            num_poss += 1;
        }
    }

    let mut num_poss_2 = 0;
    let mut triangle1: Vec<usize> = Vec::new();
    let mut triangle2: Vec<usize> = Vec::new();
    let mut triangle3: Vec<usize> = Vec::new();

    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        let lengths: Vec<usize> = params.iter().map(|x| x.parse::<usize>().unwrap()).collect();

        if triangle1.len() < 2 {
            triangle1.push(lengths[0]);
            triangle2.push(lengths[1]);
            triangle3.push(lengths[2]);
        }
        else {
            triangle1.push(lengths[0]);
            triangle2.push(lengths[1]);
            triangle3.push(lengths[2]);
            
            triangle1.sort();
            triangle2.sort();
            triangle3.sort();

            if triangle1[0] + triangle1[1] > triangle1[2] {
                num_poss_2 += 1;
            }
            if triangle2[0] + triangle2[1] > triangle2[2] {
                num_poss_2 += 1;
            }
            if triangle3[0] + triangle3[1] > triangle3[2] {
                num_poss_2 += 1;
            }

            triangle1.clear();
            triangle2.clear();
            triangle3.clear();
        }
    }

    println!("{} possible triangles", num_poss);
    println!("{} alternate possible triangles", num_poss_2);
}
