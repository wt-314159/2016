#[allow(unused_imports)]
use std::{cmp::{max, min, Reverse}, collections::HashMap, fs, hash::Hash};
use priority_queue::PriorityQueue;
use md5::{Md5, Digest};
// use fancy_regex::Regex;
// use regex::Regex;

const INPUT: &str ="hhhxzeay";
const OPEN_CHARS: [char; 5] = ['b', 'c', 'd', 'e', 'f'];

fn main() {
    // Part 1
    if let Some(directions) = a_star_search() {
        println!("Fastest path: '{}'", directions_to_string(&directions));
    }
    else {
        println!("No solution found!");
    }

    // Part 2
    let path_length = z_star_search();
    println!("Longest path took {path_length} steps");
}

fn z_star_search() -> usize {
    // find the longest path that reaches the goal
    let mut queue: PriorityQueue<Room, usize> = PriorityQueue::new();
    let start = Room::new(0, 0, Vec::new());
    let start_cost = start.estimate_cost();
    queue.push(start, start_cost);

    let mut longest = 0;
    while queue.len() > 0 {
        let (room, cost) = queue.pop().unwrap();
        if room.is_goal() {
            if cost > longest {
                longest = cost;
            }
            // can't continue after reaching end
            continue;
        }

        let open_doors = room.find_open_doors();
        for dir in open_doors {
            if let Some(new_room) = room.create_new(dir) {

                update_z_queue(&mut queue, new_room);
            }
        }
    }
    longest
}

fn update_z_queue(queue: &mut PriorityQueue<Room, usize>, new_room: Room) {
    let cost = new_room.estimate_cost();
    // add room if not in queue
    if let None = queue.get_priority(&new_room) {
        queue.push(new_room, cost);
    }
}

fn a_star_search() -> Option<Vec<Direction>> {
    let mut queue: PriorityQueue<Room, Reverse<usize>> = PriorityQueue::new();
    let start = Room::new(0, 0, Vec::new());
    let start_cost = start.estimate_cost();
    queue.push(start, Reverse(start_cost));

    while queue.len() > 0 {
        let (room, _) = queue.pop().unwrap();
        if room.is_goal() {
            return Some(room.steps);
        }

        let open_doors = room.find_open_doors();
        for dir in open_doors {
            if let Some(new_room) = room.create_new(dir) {
                update_queue(&mut queue, new_room);
            }
        }
    }
    // have run out of options without reaching goal, return None
    None
}

fn update_queue(queue: &mut PriorityQueue<Room, Reverse<usize>>, new_room: Room) {
    let cost = new_room.estimate_cost();
    // add room if not in queue
    if let None = queue.get_priority(&new_room) {
        queue.push(new_room, Reverse(cost));
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R'
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Room {
    x: usize,
    y: usize,
    steps: Vec<Direction>
}

impl Room {
    fn new(x: usize, y: usize, steps: Vec<Direction>) -> Room {
        Room { x, y, steps} 
    }

    fn create_new(&self, direction: Direction) -> Option<Room> {
        let (x, y) = match direction {
            Direction::Up => {
                if self.y == 0 { return None }
                else { (self.x, self.y - 1) }
            }
            Direction::Down => {
                if self.y == 3 { return None }
                else { (self.x, self.y + 1) }
            }
            Direction::Left => {
                if self.x == 0 { return None }
                else { (self.x - 1, self.y) }
            }
            Direction::Right => {
                if self.x == 3 { return None }
                else { (self.x + 1, self.y) }
            }
        };
        let mut new_steps = self.steps.clone();
        new_steps.push(direction);
        Some(Room::new(x, y, new_steps ))
    }

    fn find_open_doors(&self) -> Vec<Direction> {
        let hash_input = INPUT.to_owned() + &directions_to_string(&self.steps);
        let mut hasher = Md5::new();
        hasher.update(hash_input.as_bytes());
        let hash = hasher.finalize();
        let hex = format!("{:02x}", hash);

        let mut directions = Vec::new();
        let mut index = 0;
        for c in hex.chars().take(4) {
            if let Some(dir) = char_to_direction(&c, index) {
                directions.push(dir);
            }
            index += 1;
        }
        directions
    }

    fn estimate_cost(&self) -> usize {
        let x_diff = 3 - self.x;
        let y_diff = 3 - self.y;
        x_diff + y_diff + self.steps.len()
    }

    fn is_goal(&self) -> bool {
        self.x == 3 && self.y == 3
    }
}

fn directions_to_string(directions: &Vec<Direction>) -> String {
    directions.iter().map(|x| x.to_char()).collect()
}

fn char_to_direction(c: &char, index: usize) -> Option<Direction> {
    if OPEN_CHARS.contains(c) {
        match index {
            0 => Some(Direction::Up),
            1 => Some(Direction::Down),
            2 => Some(Direction::Left),
            3 => Some(Direction::Right),
            other => panic!("Shouldn't be using index '{}'", other)
        }
    }
    else {
        None
    }
}