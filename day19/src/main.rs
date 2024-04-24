#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Md5, Digest};
// use priority_queue::PriorityQueue;

fn main() {
    let input = 3005290;
    //let input = 5;
    let mut elves = Vec::new();
    for i in 0..input {
        elves.push(Elf::new(i + 1));
    }

    let mut index = 0;
    let mut last_elf_id = 0;
    loop {
        if elves[index].num_presents == 0 {
            index = increment_index(index, input);
            continue;
        }
        let mut next_index = 0;
        if let Some((found, _)) = elves.iter().enumerate().skip(index + 1).find(|(_, x)| x.num_presents > 0) {
            next_index = found;
        }
        else if let Some(found) = elves.iter().position(|x| x.num_presents > 0) {
            // have found current elf, only one with presents left
            if found == index {
                last_elf_id = elves[index].id;
                let any_presents_count = elves.iter().filter(|x| x.num_presents > 0).count();
                println!("{} elves with presents", any_presents_count);
                break;    
            }
            next_index = found;
        }
        else {
            panic!("No elves have presents!");
        }
        //println!("Elf {} takes {} presents from Elf {}", elves[index].id, elves[next_index].num_presents, elves[next_index].id);
        elves[index].num_presents += elves[next_index].num_presents;
        elves[next_index].num_presents = 0;
        index = increment_index(next_index, input);
    }

    println!("Elf {} takes all the presents", last_elf_id);
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
    index = index + 1;
    if index == max {
        index = 0;
    }
    index
}
