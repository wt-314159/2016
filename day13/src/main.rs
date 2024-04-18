use std::mem;
#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max, mem::size_of, time::Instant};
// use fancy_regex::Regex;
// use regex::Regex;

const PUZZLE_INPUT: usize = 1362;
const USIZE_SIZE: usize = mem::size_of::<usize>() * 8;

fn main() {
    println!("usize size: {}", USIZE_SIZE);
    let now = Instant::now();
    let mut bit_count1 = 0;
    for _ in 0..100 {
        bit_count1 += count_bits(usize::MAX);
    }
    let elapsed1 = now.elapsed();

    let now2 = Instant::now();
    let mut bit_count2 = 0;
    for _ in 0..100 {
        bit_count2 +=count_bits_2(usize::MAX);
    }
    let elapsed2 = now2.elapsed();

    let now3 = Instant::now();
    let mut bit_count3 = 0; 
    for _ in 0..100 {
        bit_count3 +=  count_bits_3(usize::MAX);
    }
    let elapsed3 = now3.elapsed();

    println!("Bit count 1: {} in {:#2?}", bit_count1, elapsed1);
    println!("Bit count 2: {} in {:#2?}", bit_count2, elapsed2);
    println!("BIt count 3: {} in {:#2?}", bit_count3, elapsed3);
}

fn is_wall(x: usize, y: usize) -> bool {
    let mut val = x*x + 3*x + 2*x*y + y + y*y;
    val += PUZZLE_INPUT;
    false
}

fn count_bits(val: usize) -> i32 {
    let bits = format!("{:b}", val);
    let mut ones = 0;
    for c in bits.chars() {
        if c == '1' {
            ones += 1;
        }
    }
    ones
}

fn count_bits_2(val: usize) -> i32 {
    let mask = 1;
    let mut ones = 0;
    for i in 0..USIZE_SIZE {
        if val >> i & mask  == mask {
            ones += 1;
        }
    }
    ones
}

fn count_bits_3(mut val: usize) -> usize {
    let mask = 1;
    let mut ones = 0;
    for _ in 0..USIZE_SIZE {
        if val & mask  == mask {
            ones += 1;
        }
        val = val >> 1;
    }
    ones
}