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

    println!("Cracking password from input: '{}'", input);
    
    //let hasher = Md5::new();
    let mut password = ['_'; 8];
    print_curr_password(password);
    for _ in 0..8 {
        loop {
            let mut hasher = Md5::new();
            hasher.update(hash_input.as_bytes());
            let hash = hasher.finalize();
            let hex = format!("{:02x}", hash);
            index += 1;
            hash_input = format!("{}{}", input, index);

            if hex.starts_with("00000") {
                let index_char = hex.chars().nth(5).unwrap();
                #[allow(unused_assignments)]
                let mut pass_char = '_';
                if let Some(index) = index_char.to_digit(8) {
                    let index = index as usize;
                    if password[index] == '_' {
                        pass_char = hex.chars().nth(6).unwrap();
                        password[index] = pass_char;
                        print_curr_password(password);
                        break;
                    }
                }
            }

        }
    }

    let password: String = password.iter().collect();
    println!("Password: {}", password);
}

fn print_curr_password(password: [char; 8]) {
    print!("\r");
    for c in password {
        print!("{}", c);
    }
    let completion = password.iter().filter(|x| x != &&'_').count();
    print!(" - cracked {} out of {} characters...", completion, password.len());
    println!();
}
