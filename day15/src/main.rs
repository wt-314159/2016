#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut discs = Vec::new();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        let num_positions = params[3].parse::<usize>().unwrap();
        let curr_position = params[11].trim_matches('.').parse::<usize>().unwrap();
        discs.push(Disc::new(num_positions, curr_position, discs.len() + 1));
    }

    // Part 2, comment out following line for Part 1
    discs.push(Disc::new(11, 0, discs.len() + 1));

    let mut time = 0;
    loop {
        if discs.iter().all(|x| x.is_time_match(time)) {
            break;
        }
        time += 1;
    }
    println!("Time {} matches all discs", time);
}

struct Disc {
    frequency: usize,
    phase: usize
}

impl Disc {
    fn new(num_positions: usize, start_position: usize, disc_index: usize) -> Disc {
        let (frequency, phase) = Disc::get_frequency_and_phase(num_positions, start_position, disc_index);
        Disc { frequency, phase }
    }

    fn get_frequency_and_phase(num_positions: usize, start_position: usize, disc_index: usize) -> (usize, usize) {
        // we need: start_position + disc_offset + phase = num_positions
        // so: phase = num_positions - start_position - disc_offset
        let mut positions = num_positions;
        while (start_position + disc_index) > positions {
            positions += num_positions;
        }
        let phase = positions - start_position - disc_index;
        (num_positions, phase)
    }

    fn is_time_match(&self, time: usize) -> bool {
        time % self.frequency == self.phase
    }
}
