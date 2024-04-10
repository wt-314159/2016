#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
use md5::{Md5, Digest};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = String::from("uqwqemis");
    //let input = String::from("abc");
    let mut index = 0;
    let mut hash_input = input.clone() + &index.to_string();
    
    //let hasher = Md5::new();
    let mut password = ['_'; 8];
    for i in 0..8 {
        loop {
            let mut hasher = Md5::new();
            hasher.update(hash_input.as_bytes());
            let hash = hasher.finalize();
            let hex = format!("{:02x}", hash);
            index += 1;
            hash_input = format!("{}{}", input, index);

            if hex.starts_with("00000") {
                let index_char = hex.chars().nth(5).unwrap();
                let mut pass_char = '_';
                if let Some(index) = index_char.to_digit(8) {
                    let index = index as usize;
                    if password[index] == '_' {
                        pass_char = hex.chars().nth(6).unwrap();
                        password[index] = pass_char;
                        println!("Password character at position {} is {}", i, pass_char);
                        break;
                    }
                    println!("Index {} already filled", index);
                }
                println!("Invalid digit: {}", index_char);
            }

        }
    }

    let password: String = password.iter().collect();
    println!("Password: {}", password);
}
