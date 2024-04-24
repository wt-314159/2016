use core::num;
#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Md5, Digest};
// use priority_queue::PriorityQueue;

const NUM_ELVES: usize = 5;
const HALF_ELVES: usize = NUM_ELVES / 2;

// use elves with presents / 2 then skip numb of elves to add

fn main() {
    let mut elves = Vec::new();
    for i in 0..NUM_ELVES {
        elves.push(Elf::new(i + 1));
    }

    let mut index = 0;
    let mut last_elf_id = 0;
    let mut elves_with_presents = NUM_ELVES;
    let mut to_add = 0;
    loop {
        if elves[index].num_presents == 0 {
            let opposite = get_exact_opposite(index);
            if elves[opposite].num_presents == 0 {
                //println!("Opposite elf ({}) has no presents, removing 1 from to_add {}", elves[opposite].id, to_add);
                to_add -= 1;
            }
            index = increment_index(index);
            continue;
        }
        // Part 1
        // let next_index = get_next_index(&elves, index);
        // Part 2
        let next_index = get_opposite_index(elves_with_presents, index, to_add);
        //println!("Elf {} takes {} presents from Elf {}", elves[index].id, elves[next_index].num_presents, elves[next_index].id);
        if elves[next_index].num_presents == 0 {
            println!("Something went wrong, elf index {}, next_index {}, to_add {}, elves_with_presents {}", index, next_index, to_add, elves_with_presents);
        }
        elves[index].num_presents += elves[next_index].num_presents;
        elves[next_index].num_presents = 0;
        elves_with_presents -= 1;
        to_add += 1;
        if elves_with_presents == 1 {
            last_elf_id = elves[index].id;
            break;
        }
        //println!("Added to num_to_add, now: {}", num_to_add);
        // Part 1
        //index = increment_index(next_index, NUM_ELVES);
        // Part 2

        index = increment_index(index);
    }

    println!("Elf {} takes all the presents", last_elf_id);
}

fn get_opposite_index(elves_with_presents: usize, start_index: usize, to_add: usize)-> usize {
    let opposite = elves_with_presents / 2;
    let index = add_to_index(start_index, opposite + to_add);
    index
}

fn get_exact_opposite(start_index: usize) -> usize {
    let opposite = HALF_ELVES + start_index;
    if opposite >= NUM_ELVES {
        return opposite - NUM_ELVES;
    }
    opposite
}

fn get_next_index(elves: &Vec<Elf>, start_index: usize) -> usize {
    let mut next_index = 0;
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

fn increment_index(mut index: usize) -> usize {
    index += 1;
    if index == NUM_ELVES {
        index = 0;
    }
    index
}

fn add_to_index(mut index: usize, to_add: usize) -> usize {
    index += to_add;
    while index >= NUM_ELVES {
        index -= NUM_ELVES;
    }
    index
}

fn subtract_from_index(mut index: usize, to_subtract: usize) -> usize {
    while to_subtract > index {
        index += NUM_ELVES;
    }
    index -= to_subtract;
    index
}
