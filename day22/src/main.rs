#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Md5, Digest};
// use priority_queue::PriorityQueue;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());
    
    // Part 1
    let viable = find_viable_pairs(&input);
    println!("There are {} viable pairs", viable);

    let mut nodes = Vec::new();
    for line in input.split("\n").skip(2) {
        let node = Node::new(line);
        nodes.push(node);
    }

    let goal = nodes.iter().filter(|n| n.x == 0).max_by(|a, b| a.y.cmp(&b.y)).unwrap();
    println!("Goal node: x = {} y = {}", goal.x, goal.y);
}

fn get_steps_to_goal() {
    // think the best option is some kind of modified version of A* search algorithm
    // start at the goal node, and try and find viable swaps to make that moves the goal
    // data closer to the origin. If the current node can't move, try the nodes around it
    // and see if any of them can move, starting with nodes that are closer to the origin.
    // Work our way outwards until we find a viable swap, make that swap, and then work 
    // back to the goal node
    // Not exactly an A* search algorithm I guess, but the idea is that each move, we choose
    // the node closest to the origin to swap, and this should give us the shortest path / the
    // fewest number of steps.
    // Although I don't think this is guaranteed, we could find a local minimum, in other words,
    // we might choose an option at one point which takes us closest to the origin in that step,
    // but which leads us down a route which overall takes more steps
    // Think there are too many permutations to brute force it however
}

#[allow(dead_code)]
fn find_viable_pairs(input: &str) -> usize {
    let mut used_nodes = Vec::new();
    let mut avail_nodes = Vec::new();
    for line in input.split("\n").skip(2) {
        let node = Node::new(line);
        used_nodes.push(node);
        let node = Node::new(line);
        avail_nodes.push(node);
    }

    used_nodes.sort_by(|a, b| a.used.cmp(&b.used));
    avail_nodes.sort_by(|a, b| a.avail.cmp(&b.avail));

    let num_nodes = used_nodes.len();
    let mut viable = 0;
    let mut avail_index = 0;
    for node in used_nodes {
        // skip if node is empty
        if node.used == 0 {
            continue;
        }
        while avail_index < num_nodes && avail_nodes[avail_index].avail < node.used {
            avail_index += 1;
        }
        // have found the first node where avail is big enough, all the rest will also be big enough
        viable += num_nodes - avail_index;
        // if the nodes own available space is bigger than used, we will have over counted
        if node.avail > node.used {
            viable -= 1;
        }
    }
    viable
}

struct Node {
    x: usize,
    y: usize,
    used: usize,
    avail: usize
}

impl Node {
    fn new(line: &str) -> Node {
        let params = line.split_whitespace().collect::<Vec<&str>>();
        let slash_index = params[0].rfind('/').unwrap();
        let node_name = &params[0][slash_index..];
        let node_pos_params = node_name.split('-').collect::<Vec<&str>>();
        let x = node_pos_params[1].trim_matches('x').parse::<usize>().unwrap();
        let y = node_pos_params[2].trim_matches('y').parse::<usize>().unwrap();

        let used = params[2].trim_matches('T').parse::<usize>().unwrap();
        let avail = params[3].trim_matches('T').parse::<usize>().unwrap();
        Node { x, y, used, avail }
    }
}