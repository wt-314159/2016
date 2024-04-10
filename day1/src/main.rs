#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut directions = Vec::new();
    #[allow(unused_variables)]
    for entry in input.split(", ") {
        let amount = entry[1..].parse::<i32>().unwrap();
        let dir = match &entry[0..1] {
            "L" => Direction::Left(amount),
            "R" => Direction::Right(amount),
            other => panic!("Can't parse {}", other)
        };
        directions.push(dir);
    }

    let mut north: i32 = 0;
    let mut east: i32 = 0;
    let mut max_north = 0;
    let mut min_north = 0;
    let mut max_east = 0;
    let mut min_east = 0;
    let mut facing = 0;
    // N: 0, E: 1, S: 2, W: 3
    let mut locations: Vec<(i32, i32)> = Vec::new();
    locations.push((0,0));
    for dir in directions {
        let mut amount = 0;
        match dir {
            Direction::Left(some) => {
                facing -= 1;
                amount = some;
            }
            Direction::Right(some) => {
                facing += 1;
                amount = some;
            }
        }

        // wrap from north to west and vice versa
        if facing < 0 {
            facing += 4;
        }
        else if facing > 3 {
            facing -= 4
        }

        let prev_north = north;
        let prev_east = east;

        match facing {
            0 => north += amount,
            1 => east += amount,
            2 => north -= amount,
            3 => east -= amount,
            other => panic!("Facing unkown direction {}", other)
        }

        let mut north_diff = north - prev_north;
        if north_diff != 0 {
            let mut step = 1;
            if north_diff < 0 {
                step = -1;
                north_diff = - north_diff;
            }
            for i in 1..=north_diff {
                let n = prev_north + i * step;
                let loc = (n, east);
                if locations.contains(&loc) {
                    println!("First double visit at {} north, {} east", n, east);
                    break;
                }
                locations.push(loc);
            }
        }

        let mut east_diff = east - prev_east;
        if east_diff != 0 {
            let mut step = 1;
            if east_diff < 0 {
                step = -1;
                east_diff = - east_diff;
            }
            for i in 1..=east_diff {
                let e = prev_east + i * step;
                let loc = (north, e);
                if locations.contains(&loc) {
                    println!("First double visit at {} north, {} east", north, e);
                    break;
                }
                locations.push(loc);
            }
        }
    }

    println!("{} blocks North, {} blocks East", north, east);
}

enum Direction {
    Left(i32),
    Right(i32)
}
