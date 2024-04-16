#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut floor_one: Vec<Object> = Vec::new();
    let mut floor_two: Vec<Object> = Vec::new();
    let mut floor_three: Vec<Object> = Vec::new();
    let mut floor_four: Vec<Object> = Vec::new();
    //let mut floors = vec![floor_one, floor_two, floor_three, floor_four];

    // manually entering, just quicker to type it out than to 
    // parse the input, the input shouldn't change so no reason
    // to parse it
    floor_one.push(Object::Generator(Element::Thulium));
    floor_one.push(Object::Chip(Element::Thulium));
    floor_one.push(Object::Generator(Element::Plutonium));
    floor_one.push(Object::Generator(Element::Strontium));
    floor_two.push(Object::Chip(Element::Plutonium));
    floor_two.push(Object::Chip(Element::Strontium));
    floor_three.push(Object::Generator(Element::Promethium));
    floor_three.push(Object::Chip(Element::Promethium));
    floor_three.push(Object::Generator(Element::Ruthenium));
    floor_three.push(Object::Chip(Element::Ruthenium));

    let floor_one = Floor { items: floor_one };
    let floor_two = Floor { items: floor_two };
    let floor_three = Floor { items: floor_three };
    let floor_four = Floor { items: floor_four };

    let mut building = Building { floors: vec![floor_one, floor_two, floor_three, floor_four], elevator: 0, steps: 0 };

    print_building(&building);

    println!("Estimated cost of current state is {}", estimate_cost(&building));
}

fn estimate_cost(building: &Building) -> usize {
    // goal is to have every item on the top floor, so estimate cost by number of
    // items not on top floor, multiplied by the number of floors away they each are
    // will be an underestimate, as moving items isn't as simple as that, but should be
    // good enough as a heuristic, main thing is it doesn't overestimate cost, so we
    // know the path found will be the shortest one
    building.floors[0].items.len() * 3 +
    building.floors[1].items.len() * 2 +
    building.floors[2].items.len()
}

fn a_star_search(start_state: &Building) -> usize {
    let queue = PriorityQueue::new();
    0
}

fn find_minimum_steps(floors: &mut Vec<Vec<Object>>, elevator_floor: usize, mut current_steps: usize, mut minimum_steps: usize) -> usize {
    // Even if the next step solves the puzzle, it won't beat our best attempt so far,
    // so no point continuing, just return the current minimum
    if current_steps >= minimum_steps - 1 {
        return minimum_steps;
    }
    let next_floors = get_next_floors(elevator_floor);
    for nxt_flr in next_floors {
        // Need to choose either 1 or 2 objects from current floor, and take them to next floor
        // then check if the state is valid. If not, undo what we just did and continue to try
        // other options. If the option was valid, then check if we have completed, and if not
        // recursively call find_minimum_steps, setting result equal to minimum_steps
        // Finally, undo the option, so when we try another option, we start from same position
        let num_objects = floors[elevator_floor].len();
        for i in 0..num_objects {
            // try taking just this object 
            take_elevator(floors, elevator_floor, nxt_flr, i, None);
            current_steps += 1;
            // Check if we solved puzzle
            if is_puzzle_solved(floors) {
                // undo the action, so we can keep searching for potentially better solutions
                // N.B. don't have to reduce current steps, as it should be copied to recursive calls,
                // and not affect higher levels of the call stack
                // N.B. object moved will be last object on next floor
                take_elevator(floors, nxt_flr, elevator_floor, floors[nxt_flr].len() - 1, None);
                println!("Solution found taking {} steps", current_steps);
                // return here, no matter what alternate steps we could take at this level of recursion,
                // even if they also solved the puzzle (don't think that's even possible?) they wouldn't 
                // be any quicker anyway, still take same num of steps
                return min(minimum_steps, current_steps);
            }
            // Check if state is valid
            if is_state_valid(floors) {
                minimum_steps = find_minimum_steps(floors, nxt_flr, current_steps, minimum_steps);
            }
            // undo the action
            current_steps -= 1;
            take_elevator(floors, nxt_flr, elevator_floor, floors[nxt_flr].len() - 1, None);

            // try with 2 objects if possible
            for j in i + 1..num_objects {
                take_elevator(floors, elevator_floor, nxt_flr, i, Some(j));
                current_steps += 1;
                if is_puzzle_solved(floors) {
                    // undo action
                    take_elevator(floors, nxt_flr, elevator_floor, i, Some(j));
                    println!("Solution found taking {} steps", current_steps);
                    return min(minimum_steps, current_steps);
                }

                if is_state_valid(floors) {
                    minimum_steps = find_minimum_steps(floors, nxt_flr, current_steps, minimum_steps);
                }
                current_steps -= 1;
                take_elevator(floors, nxt_flr, elevator_floor, floors[nxt_flr].len() - 2, Some(floors[nxt_flr].len() - 1));
            }
        }
    }
    minimum_steps
}

fn is_puzzle_solved(floors: &Vec<Vec<Object>>) -> bool {
    floors[3].len() == 11 && floors.iter().take(3).all(|x| x.len() == 0)
}

fn get_next_floors(elevator_floor: usize) -> Vec<usize> {
    match elevator_floor {
        0 => vec![1],
        1 => vec![2, 0],
        2 => vec![3, 1],
        3 => vec![2],
        _ => panic!("Elevator cannot be on F{}", elevator_floor + 1)
    }
}

fn take_elevator(floors: &mut Vec<Vec<Object>>, start_floor: usize, end_floor: usize, first_item_index: usize, second_item_index: Option<usize>) {
    let floor_diff = max(start_floor, end_floor) - min(start_floor, end_floor);
    if floor_diff != 1 {
        panic!("Can't go from F{} to F{}", start_floor + 1, end_floor + 1);
    }

    // remove second item first, second index should always be greater than first
    if let Some(index) = second_item_index {
        let item = floors[start_floor].remove(index);
            floors[end_floor].push(item);
    }
    let item = floors[start_floor].remove(first_item_index);
    floors[end_floor].push(item);
}

fn is_state_valid(floors: &Vec<Vec<Object>>) -> bool {
    for floor in floors {
        let unshielded_chips = chips_without_gens(floor);
        if unshielded_chips.len() != 0 && floor_has_any_gens(floor) {
            // if there are any generators on this floor, we'll fry the unshielded chips
            return false;
        }
    }
    true
}

fn chips_without_gens(floor: &Vec<Object>) -> Vec<&Object> {
    floor.iter().filter(|x| if let Object::Chip(element) = x {
        !floor.iter().any(|x| if let Object::Generator(gen_elem) = x {
            gen_elem == element
        }
    else {false })
    }
    else {false}).collect()
}

fn floor_has_any_gens(floor: &Vec<Object>) -> bool {
    floor.iter().any(|x| if let Object::Generator(_) = x { true } else {false })
}


fn print_floors(floors: &Vec<Vec<Object>>) {
    for i in (0..4).rev() {
        let floor = &floors[i];
        println!("F {} {} {} {} {} {} {} {} {} {} {}", 
        i + 1,
        get_obj_str_or_empty(floor, Object::Chip(Element::Thulium)),
        get_obj_str_or_empty(floor, Object::Generator(Element::Thulium)),
        get_obj_str_or_empty(floor, Object::Chip(Element::Plutonium)),
        get_obj_str_or_empty(floor, Object::Generator(Element::Plutonium)),
        get_obj_str_or_empty(floor, Object::Chip(Element::Strontium)),
        get_obj_str_or_empty(floor, Object::Generator(Element::Strontium)),
        get_obj_str_or_empty(floor, Object::Chip(Element::Promethium)),
        get_obj_str_or_empty(floor, Object::Generator(Element::Promethium)),
        get_obj_str_or_empty(floor, Object::Chip(Element::Ruthenium)),
        get_obj_str_or_empty(floor, Object::Generator(Element::Ruthenium)));
    }
}

fn print_building(building: &Building) {
    for i in (0..building.floors.len()).rev() {
        let floor = &building.floors[i];
        println!("F{} {} {} {} {} {} {} {} {} {} {} {}",
        i + 1,
        elevator_on_floor(building.elevator, i),
        get_obj_str_or_empty(&floor.items, Object::Chip(Element::Thulium)),
        get_obj_str_or_empty(&floor.items, Object::Generator(Element::Thulium)),
        get_obj_str_or_empty(&floor.items, Object::Chip(Element::Plutonium)),
        get_obj_str_or_empty(&floor.items, Object::Generator(Element::Plutonium)),
        get_obj_str_or_empty(&floor.items, Object::Chip(Element::Strontium)),
        get_obj_str_or_empty(&floor.items, Object::Generator(Element::Strontium)),
        get_obj_str_or_empty(&floor.items, Object::Chip(Element::Promethium)),
        get_obj_str_or_empty(&floor.items, Object::Generator(Element::Promethium)),
        get_obj_str_or_empty(&floor.items, Object::Chip(Element::Ruthenium)),
        get_obj_str_or_empty(&floor.items, Object::Generator(Element::Ruthenium)));
    }
}

fn elevator_on_floor(elevator_floor: usize, curr_floor: usize) -> String {
    if elevator_floor == curr_floor {
        String::from("E  ")
    }
    else {
        String::from(".  ")
    }
}

fn get_obj_str_or_empty(floor: &Vec<Object>, object: Object) -> String {
    if floor.contains(&object) {
        return match object {
            Object::Chip(Element::Thulium) => String::from("ThM"),
            Object::Chip(Element::Plutonium) => String::from("PlM"),
            Object::Chip(Element::Strontium) => String::from("StM"),
            Object::Chip(Element::Promethium) => String::from("PrM"),
            Object::Chip(Element::Ruthenium) => String::from("RuM"),
            Object::Generator(Element::Thulium) => String::from("ThG"),
            Object::Generator(Element::Plutonium) => String::from("PlG"),
            Object::Generator(Element::Strontium) => String::from("StG"),
            Object::Generator(Element::Promethium) => String::from("PrG"),
            Object::Generator(Element::Ruthenium) => String::from("RuG")
        }
    } 
    String::from(".  ")
}

struct Building {
    floors: Vec<Floor>,
    elevator: usize,
    steps: usize
}

impl Building {
    fn estimate_total_cost(&self) -> usize {
        let h = estimate_cost(self);
        self.steps + h
    }
}

struct Floor {
    items: Vec<Object>
}

#[derive(PartialEq, Debug)]
enum Object {
    Chip(Element),
    Generator(Element)
}

#[derive(PartialEq, Debug)]
enum Element {
    Thulium,
    Plutonium,
    Strontium,
    Promethium,
    Ruthenium
}
