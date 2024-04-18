use std::mem;
#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::{min, max, Reverse}, mem::size_of, time::Instant, hash::Hash};
use priority_queue::PriorityQueue;
// use fancy_regex::Regex;
// use regex::Regex;

const PUZZLE_INPUT: usize = 1362;
const GOAL: (usize, usize) = (31, 39);
// Test Input
// const PUZZLE_INPUT: usize = 10;
// const GOAL: (usize, usize) = (7, 4);
const USIZE_SIZE: usize = mem::size_of::<usize>() * 8;

fn main() {
    // Part 1
    let start = Square { x: 1, y: 1, steps: 0 };
    if let Some(num_steps) = a_star_search(start) {
        println!("Solution found, {} steps", num_steps);
    }
    else {
        println!("No solution found!");
    }

    // Part 2
    let start = Square { x: 1, y: 1, steps: 0 };
    let num_squares_under_50 = dijkstra(start);
    println!("Num squares under 50 steps away: {}", num_squares_under_50);
}

fn dijkstra(start: Square) -> usize {
    let mut num_squares = 1;
    let mut unmapped: PriorityQueue<Square, Reverse<usize>> = PriorityQueue::new();
    let mut mapped: HashMap<Square, usize> = HashMap::new();
    unmapped.push(start, Reverse(0));

    while unmapped.len() > 0 {
        let (square, Reverse(steps)) = unmapped.pop().unwrap();
        let neighbour_steps = square.steps + 1;
        if neighbour_steps > 50 {
            mapped.insert(square, steps);
            continue;
        }

        if !is_wall(square.x, square.y + 1) {
            let new_square = Square { x: square.x, y: square.y + 1, steps: neighbour_steps };
            update_mapped(&mut mapped, &mut unmapped, new_square);
        }
        if !is_wall(square.x + 1, square.y) {
            let new_square = Square { x: square.x + 1, y: square.y, steps: neighbour_steps };
            update_mapped(&mut mapped, &mut unmapped, new_square);
        }
        if square.y > 0 && !is_wall(square.x, square.y - 1) {
            let new_square = Square { x: square.x, y: square.y - 1, steps: neighbour_steps };
            update_mapped(&mut mapped, &mut unmapped, new_square);
        }
        if square.x > 0 && !is_wall(square.x - 1, square.y) {
            let new_square = Square { x: square.x - 1, y: square.y, steps: neighbour_steps };
            update_mapped(&mut mapped, &mut unmapped, new_square);
        }
        mapped.insert(square, steps);
    }

    mapped.iter().filter(|(_, steps)| **steps <= 50).count()
}

fn update_mapped(mapped: &mut HashMap<Square, usize>, unmapped: &mut PriorityQueue<Square, Reverse<usize>>, new_square: Square) {
    if mapped.contains_key(&new_square) {
        // Check if we've found a better path to the square
        if new_square.steps < mapped[&new_square] {
            let steps = new_square.steps;
            mapped.insert(new_square, steps);
        }
    }
    // if already in unmapped, check if our path is better
    else if let Some(steps) = unmapped.get_priority(&new_square) {
        if new_square.steps < steps.0 {
            unmapped.change_priority(&new_square, Reverse(new_square.steps));
        }
    }
    else {
        let steps = new_square.steps;
        unmapped.push(new_square, Reverse(steps));
    }
}

fn a_star_search(start: Square) -> Option<usize> {
    let mut queue: PriorityQueue<Square, Reverse<usize>> = PriorityQueue::new();
    let mut mapped: HashMap<Square, usize> = HashMap::new();
    let start_cost = estimate_cost(&start);
    queue.push(start, Reverse(start_cost));

    while queue.len() > 0 {
        let (square, cost) = queue.pop().unwrap();
        if square.is_goal() {
            return Some(square.steps);
        }

        let neighbour_steps = square.steps + 1;
        if !is_wall(square.x, square.y + 1) {
            let new_square = Square { x: square.x, y: square.y + 1, steps: neighbour_steps };
            update_queue(&mapped, &mut queue, new_square, neighbour_steps);
        }
        if !is_wall(square.x + 1, square.y) {
            let new_square = Square { x: square.x + 1, y: square.y, steps: neighbour_steps };
            update_queue(&mapped, &mut queue, new_square, neighbour_steps);
        }
        if square.y > 0 && !is_wall(square.x, square.y - 1) {
            let new_square = Square { x: square.x, y: square.y - 1, steps: neighbour_steps };
            update_queue(&mapped, &mut queue, new_square, neighbour_steps);
        }
        if square.x > 0 && !is_wall(square.x - 1, square.y) {
            let new_square = Square { x: square.x - 1, y: square.y, steps: neighbour_steps };
            update_queue(&mapped, &mut queue, new_square, neighbour_steps);
        }
        mapped.insert(square, cost.0);
    }
    None
}

fn update_queue(mapped: &HashMap<Square, usize>, queue: &mut PriorityQueue<Square, Reverse<usize>>, new_square: Square, steps: usize) {
    let est_cost = estimate_cost(&new_square);

    if let Some(queue_steps) = queue.get_priority(&new_square) {
        if steps + est_cost < queue_steps.0 {
            queue.change_priority(&new_square, Reverse(steps + est_cost));
        }
    }
    else if !mapped.contains_key(&new_square) {
        queue.push(new_square, Reverse(steps + est_cost));
    }
}

fn estimate_cost(square: &Square) -> usize {
    let x_diff = max(GOAL.0, square.x) - min(GOAL.0, square.x);
    let y_diff = max(GOAL.1, square.y) - min(GOAL.1, square.y);
    x_diff + y_diff
}

fn print_maze(maze: Vec<Vec<bool>>) {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            let c = match maze[i][j] {
                true => '#',
                false => '.'
            };
            print!("{c}");
        }
        println!();
    }
}

fn is_wall(x: usize, y: usize) -> bool {
    let mut val = x*x + 3*x + 2*x*y + y + y*y;
    val += PUZZLE_INPUT;
    let bits = count_bits(val);
    bits % 2 != 0
}

fn count_bits(mut val: usize) -> usize {
    let mask = 1;
    let mut ones = 0;
    for _ in 0..USIZE_SIZE {
        if val & mask  == mask {
            ones += 1;
        }
        val = val >> 1;
    }
    ones
}

struct Square {
    steps: usize,
    x: usize,
    y: usize
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Square {}

impl Hash for Square {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Square {
    fn is_goal(&self) -> bool {
        self.x == GOAL.0 && self.y == GOAL.1
    }
}