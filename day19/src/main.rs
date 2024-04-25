#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Md5, Digest};
// use priority_queue::PriorityQueue;

const INPUT: usize = 3005290;
#[allow(dead_code)]
const HALF_ELVES: usize = INPUT / 2;

// use elves with presents / 2 then skip numb of elves to add

fn main() {
    // have found pattern in winning elf id's, based on number of elves
    // starting with 2 elves, where the winner is trivially 1,
    // the winning elf id increments by 1 each time, until the winning elf id
    // equals the last number of elves where the pattern reset (more on this later), 
    // at which point that number is missed, and every even number is missed from 
    // this point on, so the winning elf id increements by 2, until the winning elf id
    // matches the number of elves, then the pattern resets.
    // hard to explain, but run find_pattern() and you'll see.
    // The number at which the number of elves matches the winning elf id triples each time,
    // starting on 1, then 3, then 9 etc.

    //find_pattern();

    let winner = find_solution_by_pattern(INPUT);
    println!("Winning elf is {}", winner);
}

fn find_solution_by_pattern(input: usize) -> usize {
    let mut last_num_elves = 1;
    let mut num_elves = 1;
    while num_elves < input {
        last_num_elves = num_elves;
        num_elves *= 3;
    }
    let transition = last_num_elves * 2;
    if transition > input {
        return input - last_num_elves;
    }
    else {
        return transition - last_num_elves + 2 * (input - transition);
    }
}

#[allow(dead_code)]
fn find_pattern() {
    for i in 2..1000 {
        let winning_elf_id = find_winning_elf(i);
        println!("{i:<10} {winning_elf_id}");
        // if winning_elf_id == i {
        //     println!("{i}");
        // }
    }
}

#[allow(dead_code)]
fn find_winning_elf(num_elves: usize) -> usize {
    let mut elves = Vec::new();
    for i in 0..num_elves {
        elves.push(Elf::new(i + 1));
    }

    let mut index = 0;
    let last_elf_id;
    let mut elves_with_presents = num_elves;
    loop {
        if elves[index].num_presents == 0 {
            index = increment_index(index, num_elves);
            continue;
        }
        // Part 1
        // let next_index = get_next_index(&elves, index);
        // Part 2
        let next_index = get_opposite_index(&elves, elves_with_presents, index);
        //println!("Elf {} takes {} presents from Elf {}", elves[index].id, elves[next_index].num_presents, elves[next_index].id);
        if elves[next_index].num_presents == 0 {
            println!("Something went wrong, elf index {}, next_index {}, elves_with_presents {}", index, next_index, elves_with_presents);
        }
        elves[index].num_presents += elves[next_index].num_presents;
        elves[next_index].num_presents = 0;
        elves_with_presents -= 1;
        if elves_with_presents == 1 {
            last_elf_id = elves[index].id;
            break;
        }
        //println!("Added to num_to_add, now: {}", num_to_add);
        // Part 1
        //index = increment_index(next_index, NUM_ELVES);
        // Part 2

        index = increment_index(index, num_elves);
    }
    last_elf_id
}

fn get_opposite_index(elves: &Vec<Elf>, elves_with_presents: usize, start_index: usize)-> usize {
    let opposite = elves_with_presents / 2;
    let mut counter = 0;
    let mut index = start_index;
    while counter < opposite {
        index = increment_index(index, elves.len());
        if elves[index].num_presents != 0 {
            counter += 1;
        }
    }
    index
}

#[allow(dead_code)]
fn get_next_index(elves: &Vec<Elf>, start_index: usize) -> usize {
    let next_index;
    if let Some((found, _)) = elves.iter().enumerate().skip(start_index + 1).find(|(_, x)| x.num_presents > 0) {
        next_index = found;
    }
    else if let Some(found) = elves.iter().position(|x| x.num_presents > 0) {
        // have found current elf, only one with presents left
        next_index = found;
    }
    else {
        panic!("No elves have presents!");
    }
    next_index
}

struct Elf {
    id: usize,
    num_presents: usize
}

impl Elf {
    fn new(id: usize) -> Elf {
        Elf { id, num_presents: 1 }
    }
}

fn increment_index(mut index: usize, max: usize) -> usize {
    index += 1;
    if index == max {
        index = 0;
    }
    index
}
