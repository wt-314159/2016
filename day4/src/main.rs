#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

const ALPHABET: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut sector_sum = 0;
    let mut valid_rooms = 0;
    let mut possible_answers = Vec::new();
    for line in input.split("\n") {
        let checksum_start = line.chars().enumerate().find(|x| x.1 == '[').unwrap();
        let checksum = &line[checksum_start.0 + 1..line.len() - 1];
        let sector_start = line[..checksum_start.0].rfind('-').unwrap();
        let sector = &line[sector_start + 1..checksum_start.0];

        let mut char_counts: Vec<CharCount> = Vec::new();
        for c in line[0..sector_start].chars() {
            if c == '-' {
                continue;
            }
            if let Some((index, c_count)) = char_counts.iter().enumerate().find(|x| x.1.0 == c) {
                char_counts[index].1 += 1;
            }
            else {
                char_counts.push(CharCount(c,1));
            }
        }

        char_counts.sort();
        let mut checks_match = true;
        for (i, char) in char_counts.iter().enumerate().take(5) {
            if checksum.chars().nth(i).unwrap() != char.0 {
                checks_match = false;
                break;
            }
        }
        if checks_match {
            valid_rooms += 1;
            let sector_num = sector.parse::<usize>().unwrap();
            sector_sum += sector_num;
            let room_name = decrypt_name(&line[0..sector_start], sector_num);
            println!("Decrypted room name: '{}' - ID: {}", room_name, sector);
            if room_name.contains("storage") || room_name.contains("store") || room_name.contains("north pole") {
                possible_answers.push((room_name, sector));
            }
        }
    }

    println!("Total valid sector sum: {}", sector_sum);

    for poss_ans in possible_answers {
        println!("Possible room solution: {} - ID: {}", poss_ans.0, poss_ans.1);
    }
}

fn decrypt_name(encrypted: &str, sector: usize) -> String {
    let mut decrypted: Vec<char> = Vec::new();
    for c in encrypted.chars() {
        match c {
            '-' => decrypted.push(' '),
            _ => decrypted.push(get_shifted_char(&c, sector))
        }
    }
    decrypted.iter().collect()
}

fn get_shifted_char(c: &char, shift: usize) -> char {
    let index = ALPHABET.iter().position(|x| x == c).unwrap();
    let new_index = (index + shift) % 26;
    ALPHABET[new_index]
}

#[derive(Debug, Eq, PartialEq, Ord)]
struct CharCount(char, usize);

impl PartialOrd for CharCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.1 == other.1 {
            return Some(self.0.cmp(&other.0));
        }
        // reverse sort by number 
        return Some(other.1.cmp(&self.1));
    }
}
