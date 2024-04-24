#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Md5, Digest};
// use priority_queue::PriorityQueue;

const NUM_ELVES: usize = 3005290;

fn main() {
    let mut elves = Vec::new();
    for i in 0..NUM_ELVES {
        elves.push(Elf::new(i + 1));
    }

    let mut index = 0;
    let mut last_elf_id = 0;
    let mut elves_with_presents = NUM_ELVES;
    let mut num_to_add = 0;
    loop {
        if elves[index].num_presents == 0 {
            index = increment_index(index, NUM_ELVES);
            num_to_add -= 1;
            continue;
        }
        // Part 1
        // let next_index = get_next_index(&elves, index);
        // Part 2
        let next_index = get_opposite_index(&elves, index, elves_with_presents);
        if next_index == index {
            last_elf_id = elves[index].id;
            let any_presents_count = elves.iter().filter(|x| x.num_presents > 0).count();
            println!("{} elves with presents", any_presents_count);
            break;    
        }
        //println!("Elf {} takes {} presents from Elf {}", elves[index].id, elves[next_index].num_presents, elves[next_index].id);
        elves[index].num_presents += elves[next_index].num_presents;
        elves[next_index].num_presents = 0;
        num_to_add += 1;
        // Part 1
        //index = increment_index(next_index, NUM_ELVES);
        // Part 2
        index = increment_index(index, NUM_ELVES);
        elves_with_presents -= 1;
        if elves_with_presents == 1 {
            last_elf_id = elves[index].id;
        }
    }

    println!("Elf {} takes all the presents", last_elf_id);
}

fn get_opposite_index(elves: &Vec<Elf>, start_index: usize, elves_with_presents: usize)-> usize {
    let opposite = elves_with_presents / 2;
    let mut index = start_index;
    let mut count = 0;
    while count < opposite {
        index = increment_index(index, NUM_ELVES);
        if elves[index].num_presents > 0 {
            count += 1;
        }
    }
    index
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

fn increment_index(mut index: usize, max: usize) -> usize {
    index += 1;
    if index == max {
        index = 0;
    }
    index
}
