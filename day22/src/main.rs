#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::{min, max, Reverse, Ordering}};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Md5, Digest};
use priority_queue::PriorityQueue;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());
    
    // Part 1
    //let viable = find_viable_pairs(&input);
    //println!("There are {} viable pairs", viable);

    let test = get_steps_to_goal(&input);
}

fn get_steps_to_goal(input: &str) -> usize {
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

    // First of all, need to arrange the nodes in some way so we can easily find neighbours etc
    // So, start with the goal node, find it's used size, then add each of it's neighbours to a priority queue,
    // ordered by which ones are closest to the origin. Go through the priority queue, and check if any of them
    // have enough avail space for the goal node. If it does, swap the nodes, and continue. If not, then get the
    // used size of the current node, and see if any of it's neighbours have enough avail space/
    // Once we've made a swap, we need to work back to the goal node, swapping on our way. So start at the recently
    // freed node, and add it's neighbours to a (different?) priority queue, this time prioritised by how close 
    // they are to the goal node, and see if they have little enough used space to swap into the current free node.
    // repeat until we reach the goal node, or none of the neighbours have enough space
    // Then start working back outwards to find the next node that can move again 

    // 
    let mut to_recalculate: Vec<(usize, usize)> = Vec::new();
    let mut nodes: [[Option<Node>; 32]; 30] = [[None; 32]; 30];
    for line in input.split("\n").skip(2) {
        let node = Node::new(line);
        nodes[node.x][node.y] = Some(node);
        to_recalculate.push((node.x, node.y));
    }

    loop {
        // have to find available swaps, and choose the best swap to make out of all possible
        let mut swaps = find_swaps(&to_recalculate, &nodes, (0, 31));
        let best_swap = swaps.pop().unwrap().0;
        // make the swap, then add the nieghbours to the recalculate list
        to_recalculate = find_swap_neighbours(&best_swap);
        swap_nodes(&mut nodes, best_swap);
        panic!("Not implemented yet! Need to work out how to decide what the goal node is and how to prioritise swaps");
    }

    0
}

fn find_swap_neighbours((source_node, target_node): &(Node, Node)) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let (x_min, x_max) = min_max(source_node.x, target_node.x);
    let (y_min, y_max) = min_max(source_node.y, target_node.y);
    let x_min = subtract_if_possible(x_min);
    let y_min = subtract_if_possible(y_min);
    let x_max = min(x_max + 1, 29);
    let y_max = min(y_max + 1, 31);
    for x_i in x_min..=x_max {
        for y_i in y_min..=y_max {
            neighbours.push((x_i, y_i));
        }
    }
    neighbours
}

fn subtract_if_possible(v: usize) -> usize {
    match v {
        0 => 0,
        x => x - 1
    }
}

#[inline]
fn min_max<T>(v1: T, v2: T) -> (T, T) 
where T: std::cmp::Ord {
    match Ord::cmp(&v1, &v2) {
        Ordering::Less | Ordering::Equal => (v1, v2),
        Ordering::Greater => (v2, v1)
    }
}

fn swap_nodes(nodes: &mut [[Option<Node>; 32]; 30], (source_node, target_node): (Node, Node)) {
    nodes[target_node.x][target_node.y].unwrap().add_used(source_node.used);
    nodes[source_node.x][source_node.y].unwrap().empty();
}

fn print_swaps(swaps: &PriorityQueue<(Node, Node), Reverse<usize>>) {
    for ((source_node, target_node), Reverse(len)) in swaps {
        println!("(x: {:>2}, y: {:>2}) -> (x: {:>2}, y: {:>2})      distance: {:>2}     used: {:>2}     avail: {:>2}", 
        source_node.x, source_node.y, target_node.x, target_node.y, len, source_node.used, target_node.avail);
    }
}

fn find_swaps(to_recalculate: &Vec<(usize, usize)>, nodes: &[[Option<Node>; 32]; 30], target: (usize, usize)) -> PriorityQueue<(Node, Node), Reverse<usize>> {
    let mut swaps = PriorityQueue::new();
    for (x, y) in to_recalculate {
        let target_node = nodes[*x][*y].unwrap();
        let neighbours = find_neighbours(nodes, *x, *y);
        for n in neighbours {
            if n.used < target_node.avail {
                let distance = n.distance_to_target(target);
                swaps.push((n, target_node), Reverse(distance));
            }
        }
    }
    swaps
}

fn find_neighbours(nodes: &[[Option<Node>; 32]; 30], x: usize, y: usize) -> Vec<Node> {
    let mut neighbours = Vec::new();
    let x_min = if x > 0 { x - 1 } else { 0 };
    let y_min = if y > 0 { y - 1 } else { 0 };
    let x_max = min(x + 1, 29);
    let y_max = min(y + 1, 31);
    for x_i in x_min..=x_max {
        for y_i in y_min..=y_max {
            if x_i == x && y_i == y { continue; }
            neighbours.push(nodes[x_i][y_i].unwrap());
        }
    }
    neighbours
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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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

    fn distance_to_target(&self, target: (usize, usize)) -> usize {
        let (x, y) = target;
        let min_x = min(self.x, x);
        let min_y = min(self.y, y);
        let max_x = max(self.x, x);
        let max_y = max(self.y, y);
        (max_x - min_x) + (max_y - min_y)
    }

    fn add_used(&mut self, used: usize) {
        self.used += used;
        self.avail -= used;
    }

    fn empty(&mut self) {
        self.avail += self.used;
        self.used = 0;
    }
}