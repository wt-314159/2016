#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
//use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut counter = 0;
    let mut ssl_counter = 0;
    for line in input.split("\n") {
        let supports_tls = supports_tls(line);
        if supports_tls {
            counter += 1;
        }
        let supports_sll = supports_ssl(line);
        if supports_sll {
            ssl_counter += 1;
        }
    }

    println!("TLS count: {}", counter);
    println!("SSL count: {}", ssl_counter);
}

fn supports_tls(ip_addr: &str) -> bool {
    let mut in_brackets = false;
    let mut supports_tls = false;
    let mut last_char = '_';
    let mut last_char_2 = '-';
    let mut last_char_3 = '=';
    for c in ip_addr.chars() {
        if c == '[' {
            in_brackets = true;
            last_char = '_';
            last_char_2 = '2';
            last_char_3 = '3';
            continue;
        }
        else if c == ']' {
            in_brackets = false;
            last_char = '_';
            last_char_2 = '2';
            last_char_3 = '3';
            continue;
        }
        if last_char_3 == c && last_char_2 == last_char && last_char != c {
            if in_brackets {
                return false;
            }
            supports_tls = true;
        }
        last_char_3 = last_char_2;
        last_char_2 = last_char;
        last_char = c;
    }
    supports_tls
}

fn supports_ssl(ip_addr: &str) -> bool {
    let mut in_brackets = false;
    let mut supports_ssl = false;
    let mut prev_char = '_';
    let mut prev_char_2 = ';';
    let mut abas: Vec<String> = Vec::new();
    let mut babs: Vec<String> = Vec::new();

    for c in ip_addr.chars() {
        if c == '[' {
            in_brackets = true;
            prev_char = '_';
            prev_char_2 = ';';
        }
        else if c == ']' {
            in_brackets = false;
            prev_char = '_';
            prev_char_2 = ';';
        }
        else {
            if c == prev_char_2 && c != prev_char {
                if in_brackets {
                    let aba: String = [prev_char, c, prev_char].iter().collect();
                    babs.push(aba);
                }
                else {
                    let aba: String = [prev_char_2, prev_char, c].iter().collect();
                    abas.push(aba);
                }
            }
            prev_char_2 = prev_char;
            prev_char = c;
        }
    }

    for aba in babs {
        if abas.contains(&aba) {
            supports_ssl = true;
        }
    }
    supports_ssl
}
