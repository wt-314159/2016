#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Md5, Digest};
// use priority_queue::PriorityQueue;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    //println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut blocked_ips = Vec::new();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split('-').collect();
        let start = params[0].parse::<u32>().unwrap();
        let end = params[1].parse::<u32>().unwrap();
        blocked_ips.push(Blocked {start, end});
    }

    blocked_ips.sort_by(|a, b| a.start.cmp(&b.start));
    let mut blocking: HashMap<u32, &Blocked> = HashMap::new();
    // add first to blocking manually, so we can start loop at 1 and not have to worry about
    // trying to subtract 1 from index when index is 0 and index is usize
    blocking.insert(blocked_ips[0].end, &blocked_ips[0]);
    let mut next_blocked_index = 1;
    let mut next_blocked = &blocked_ips[1];
    let mut blocking_count = 1;
    // Part 2
    let mut unblocked_ips = 0;
    let mut min_blocking_end = blocked_ips[0].end;
    let mut index = 1;
    while index <= u32::MAX {
        if next_blocked.start == index {
            blocking.insert(next_blocked.end, next_blocked);
            if next_blocked.end < min_blocking_end {
                min_blocking_end = next_blocked.end;
            }
            blocking_count += 1;
            next_blocked_index += 1;
            if next_blocked_index == blocked_ips.len() {
                break;
            }
            next_blocked = &blocked_ips[next_blocked_index];
        }
        let prev_index = index - 1;
        if blocking.contains_key(&prev_index) {
            blocking.remove(&prev_index);
            blocking_count -= 1;
            if prev_index == min_blocking_end {
                if blocking_count > 0 {
                    min_blocking_end = *blocking.iter().min_by(|a, b| a.0.cmp(b.0)).unwrap().0;
                }
                else {
                    min_blocking_end = u32::MAX;
                }
            }
        }

        let next_index = min(next_blocked.start - 1, min_blocking_end);
        if blocking_count == 0 {
            // Part 1
            //println!("First unblocked ip is at i: {}", i);
            //break;
            // Part 2
            unblocked_ips += (next_index+ 1) - index;
        }
        index = next_index + 1;
    }

    let last_blocking = blocking.iter().max_by(|(_, a), (_, b)| a.end.cmp(&b.end)).unwrap();
    let num_end = u32::MAX - last_blocking.1.end;
    unblocked_ips += num_end;

    println!("There are {} unblocked ips, including {} at the end", unblocked_ips, num_end);
}

struct Blocked {
    start: u32,
    end: u32
}