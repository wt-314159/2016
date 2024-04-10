#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut actions = Vec::new();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();

        let action = match params[0] {
            "rotate" => {
                let index = params[2].split("=").last().unwrap().parse::<usize>().unwrap();
                let by = params[4].parse::<usize>().unwrap();
                match params[1] {
                    "column" => Action::RotateColumn(index, by),
                    "row" => Action::RotateRow(index, by),
                    other => panic!("Do not recognise {}", other)
                }
            },
            "rect" => {
                let lengths: Vec<&str> = params[1].split("x").collect();
                let width = lengths[0].parse::<usize>().unwrap();
                let height = lengths[1].parse::<usize>().unwrap();
                Action::Rect(width, height)
            },
            other => panic!("Can't parse {}", other)
        };

        actions.push(action);
    }

    let mut lights = [[false; 50]; 6];

    for act in actions {
        match act {
            Action::Rect(width, height) => turn_on_lights(&mut lights, width, height),
            Action::RotateRow(row, by) => rotate_row(&mut lights, row, by),
            Action::RotateColumn(col, by) => rotate_col(&mut lights, col, by)
        }
    }

    println!("All finished");
    let num_on = lights.concat().iter().filter(|x| **x).count();
    println!("Number of pixels lit: {}", num_on);

    println!();
    print_screen(&lights);
}

fn turn_on_lights(lights: &mut [[bool; 50]; 6], width: usize, height: usize) {
    for i in 0..width {
        for j in 0..height {
            lights[j][i] = true;
        }
    }
}

fn rotate_row(lights: &mut [[bool; 50]; 6], row: usize, by: usize) {
    let curr_light = lights[row];
    for i in 0..49 {
        let target_index = get_bounded_index(i, by, 50);
        lights[row][i] = curr_light[target_index];
    }
}

fn rotate_col(lights: &mut [[bool; 50]; 6], col: usize, by: usize) {
    let curr_lights = get_column(lights, col);
    for j in 0..6 {
        let target_index = get_bounded_index(j, by, 6);
        lights[j][col] = curr_lights[target_index];
    }
}

fn get_bounded_index(index: usize, by: usize, max: usize) -> usize {
    let mut target_index = index;
    while target_index < by {
        target_index += max;
    }
    target_index -= by;
    target_index
}

fn get_column(lights: &[[bool; 50]; 6], col: usize) -> [bool;6] {
    let mut light_col = [false; 6];
    for r in 0..6 {
        light_col[r] = lights[r][col];
    }
    light_col
}

fn print_screen(lights: &[[bool; 50]; 6]) {
    for r in 0..6 {
        for c in 0..50 {
            let light = lights[r][c];
            if light {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
}

enum Action {
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
    Rect(usize, usize)
}