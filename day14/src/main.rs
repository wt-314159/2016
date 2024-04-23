#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
use md5::{Md5, Digest};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = String::from("jlmsuwbz");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut index: usize = 0;
    let mut triples: Vec<(char, usize, String)> = Vec::new();
    let mut hashes: Vec<(String, usize)> = Vec::new();

    while hashes.len() < 64 {
        // remove all potential options that we haven't found 5 repeating chars for
        if index > 1000 {
            let out_of_bounds = index - 1000;
            if let Some(first_ok) = triples.iter().enumerate().find(|(_, (_, i, _))| *i > out_of_bounds) {
                for i in (0..first_ok.0).rev() {
                    triples.remove(i);
                }
            }
        }
        let hash_input = input.clone() + &index.to_string();
        let mut hasher = Md5::new();
        hasher.update(hash_input.as_bytes());

        let hash = hasher.finalize();
        let hex = format!("{:02x}", hash);
        // Part 2, for Part 1, comment following line
        let hex = stretch_hash(&hex);

        // check for 5 repeating characters
        if let Some(quintets) = get_any_quintets(&hex) {
            let indices: Vec<usize> = triples.iter().enumerate()
            .filter(|(_, (c, _, _))| quintets.contains(c))
            .map(|(i, (_, _, _))| i).collect();
            
            for i in (0..indices.len()).rev() {
                let index = indices[i];
                let triple = triples.remove(index);
                println!("Hash {} is a match!", triple.2);
                hashes.push((triple.2, triple.1));
            }
        }
        if let Some(c) = get_first_triplet(&hex) {
            println!("Hash {} contains 3 repeating '{}'s", hex, c);
            triples.push((c, index, hex));
        }

        index += 1;
    }

    println!("{} hashes found", hashes.len());
    hashes.sort_by(|(_, i_1), (_, i_2)| i_1.cmp(i_2));
    let last = &hashes[63];
    println!("64th hash found using index {}", last.1);
}

fn get_first_triplet(hash: &str) -> Option<char> {
    let mut count = 1;
    let mut prev_char = '_';
    for c in hash.chars() {
        if c == prev_char {
            count += 1;
            if count == 3 {
                return Some(prev_char);
            }
        }
        else {
            prev_char = c;
            count = 1;
        }
    }
    None
}

fn get_any_quintets(hash: &str) -> Option<Vec<char>> {
    let mut count = 1;
    let mut prev_char = '_';
    let mut quintets = Vec::new();
    for c in hash.chars() {
        if c == prev_char {
            count += 1;
            if count == 5 {
                quintets.push(prev_char);
                println!("Hash '{}' contains 5 repeating '{}'s", hash, prev_char);
            }
        }
        else {
            prev_char = c;
            count = 1;
        }
    }
    if quintets.len() == 0 {
        None
    }
    else {
        Some(quintets)
    }
}

fn stretch_hash(hash: &str) -> String {
    let mut curr_hash = String::from(hash);
    for _ in 0..2016 {
        let mut hasher = Md5::new();
        hasher.update(curr_hash.as_bytes());
        let h = hasher.finalize();
        curr_hash = format!("{:02x}", h);
    }
    curr_hash
}
