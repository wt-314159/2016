#[allow(unused_imports)]
use std::{cmp::{max, min, Reverse}, collections::HashMap, fs, hash::Hash, io::{stdout, Write}};
use priority_queue::PriorityQueue;
use termion::{cursor, raw::IntoRawMode};
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
    // Part 2 --------------
    floor_one.push(Object::Generator(Element::Elerium));
    floor_one.push(Object::Chip(Element::Elerium));
    floor_one.push(Object::Generator(Element::Dilithium));
    floor_one.push(Object::Chip(Element::Dilithium));
    // ---------------------

    let floor_one = Floor { items: floor_one };
    let floor_two = Floor { items: floor_two };
    let floor_three = Floor { items: floor_three };
    let floor_four = Floor { items: Vec::new() };

    let building = Building { floors: vec![floor_one, floor_two, floor_three, floor_four], elevator: 0, steps: 0 };

    print_building(&building);

    println!("Estimated cost of current state is {}", estimate_cost(&building));

    if let Some(min_steps) = a_star_search(building) {
        println!("Solution found requiring {} steps", min_steps);
    }
    else {
        println!("No solution found!");
    }
}

fn estimate_cost(building: &Building) -> usize {
    // goal is to have every item on the top floor, so estimate cost by number of
    // items not on top floor, multiplied by the number of floors away they each are
    // will be an underestimate, as moving items isn't as simple as that, but should be
    // good enough as a heuristic, main thing is it doesn't overestimate cost, so we
    // know the path found will be the shortest one

    // N.B. REALLY IMPORTANT -> the choice of this heuristic massively affects A* search efficiency!
    // At first, I just used the number of items on each floor, multiplied by how many floors away from
    // floor 4 that floor was. However this is a big underestimate of the number of steps required to
    // get to the finished state, and since the estimated total cost of a state is the number of steps 
    // taken to get to that state plus the estimated number of steps to get to the goal, by vastly
    // underestimating the remaining steps, the A* algorithm was prioritising states where the fewest
    // steps had been taken over states where more steps had been taken and were closer to the goal, but
    // not by much. In other words, the A* algorithm assumed earlier states were a better candidate than
    // later states, since we were underestimating the amount of steps required to get to later states,
    // and so later states didn't look appealing, as they had more steps taken than we thought they should.
    // Because of this, we were essentially checking every single state on the way, as any state that progressed
    // too far would get deprioritised over earlier states, and this was taking far far too long.
    // The fix was simply to multiply this whole estimated cost by 3, and so weight the remaining number of steps
    // more heavily, so later nodes were prioritised over earlier ones, as their estimated cost was more accurate
    (building.floors[0].items.len() * 3 +
    building.floors[1].items.len() * 2 +
    building.floors[2].items.len()) * 3
}

fn a_star_search(start_state: Building) -> Option<usize> {
    let mut queue: PriorityQueue<Building, Reverse<usize>> = PriorityQueue::new();
    let mut mapped: HashMap<Building, usize> = HashMap::new();
    let cost = estimate_cost(&start_state);
    queue.push(start_state, Reverse(cost));
    let mut stdout = stdout().into_raw_mode().unwrap();

    while queue.len() > 0 {
        let (building, cost) = queue.pop().unwrap();
        // if we've reached goal, return number of steps it took to get there
        if building.is_goal() {
            return Some(building.steps);
        }
        //writeln!(stdout, "{}{}", cursor::Goto(1, 9), building.steps).expect("Write error!");

        // find all neighbours and calculate cost
        let next_floors = get_next_floors(building.elevator);
        let num_objects = building.floors[building.elevator].items.len();
        let neighbour_steps = building.steps + 1;
        for nxt_flr in next_floors {
            for i in 0..num_objects {
                // Create a new state by moving 1 object to next floor
                let new_state = create_state(&building, building.elevator, nxt_flr, i, None);
                if new_state.is_valid() { 
                    let est_cost = estimate_cost(&new_state);
                    // if state is already in queue, check if our new path to it is faster, and update it if so
                    if let Some(queue_steps) = queue.get_priority(&new_state) {
                        if neighbour_steps + est_cost < queue_steps.0 {
                            queue.change_priority(&new_state, Reverse(neighbour_steps + est_cost));
                        }
                    }
                    // Only add new state to queue if it's not been fully mapped yet
                    else if !mapped.contains_key(&new_state) {
                        queue.push(new_state, Reverse(neighbour_steps + est_cost));
                    }
                }

                for j in i + 1..num_objects {
                    let new_state = create_state(&building, building.elevator, nxt_flr, i, Some(j));
                    if !new_state.is_valid() { continue; }
    
                    let est_cost = estimate_cost(&new_state);
                    if let Some(queue_steps) = queue.get_priority(&new_state) {
                        if neighbour_steps + est_cost < queue_steps.0 {
                            queue.change_priority(&new_state, Reverse(neighbour_steps + est_cost));
                        }
                    }
                    else if !mapped.contains_key(&new_state) {
                        queue.push(new_state, Reverse(neighbour_steps + est_cost));
                    }
                }
            }
        }
        // insert state into a hashmap of fully mapped nodes
        if mapped.contains_key(&building) {
            println!("Mapped same building twice!!");
            return Some(0);
        }
        mapped.insert(building, cost.0);
        //writeln!(stdout, "{}{}", cursor::Goto(1, 11), mapped.len()).expect("Error writing!");
    }
    // No more nodes to check, puzzle isn't solved
    None
}

fn create_state(building: &Building, start_floor: usize, dest_floor: usize, first_item_index: usize, second_item_index: Option<usize>) -> Building {
    let mut new_state = building.clone();

    if let Some(index) = second_item_index {
        let item = new_state.floors[start_floor].items.remove(index);
        new_state.floors[dest_floor].items.push(item);
    }
    let item = new_state.floors[start_floor].items.remove(first_item_index);
    new_state.floors[dest_floor].items.push(item);
    // Remember to move the elevator!
    new_state.elevator = dest_floor;
    // and update num steps
    new_state.steps += 1;
    new_state
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

fn print_building(building: &Building) {
    for i in (0..building.floors.len()).rev() {
        let floor = &building.floors[i];
        println!("F{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
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
        get_obj_str_or_empty(&floor.items, Object::Generator(Element::Ruthenium)),
        // Part 2
        get_obj_str_or_empty(&floor.items, Object::Chip(Element::Elerium)), 
        get_obj_str_or_empty(&floor.items, Object::Generator(Element::Elerium)),
        get_obj_str_or_empty(&floor.items, Object::Chip(Element::Dilithium)),
        get_obj_str_or_empty(&floor.items, Object::Generator(Element::Dilithium)));
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
            // Part 2 ------------
            Object::Chip(Element::Elerium) => String::from("ElM"),
            Object::Chip(Element::Dilithium) => String::from("DlM"), 
            // -------------------
            Object::Generator(Element::Thulium) => String::from("ThG"),
            Object::Generator(Element::Plutonium) => String::from("PlG"),
            Object::Generator(Element::Strontium) => String::from("StG"),
            Object::Generator(Element::Promethium) => String::from("PrG"),
            Object::Generator(Element::Ruthenium) => String::from("RuG"),
            // Part 2
            Object::Generator(Element::Elerium) => String::from("ElG"),
            Object::Generator(Element::Dilithium) => String::from("DlG"),
        }
    } 
    String::from(".  ")
}

#[derive(Clone)]
struct Building {
    floors: Vec<Floor>,
    elevator: usize,
    steps: usize
}

impl Building {
    fn is_goal(&self) -> bool {
        // all items on 4th floor
        // Part 2, for Part 1, use 10
        self.floors[3].items.len() == 14 && 
        // no items on any other floors (should be case )
        //self.floors.iter().take(3).all(|x| x.items.len() == 0) &&     // shouldn't need to check this
        self.elevator == 3  
    }

    fn is_valid(&self) -> bool {
        for floor in &self.floors {
            let unshielded_chips = chips_without_gens(&floor.items);
            if unshielded_chips.len() != 0 && floor_has_any_gens(&floor.items) {
                // if there are any generators on this floor, we'll fry the unshielded chips
                return false;
            }
        }
        true
    }
}

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        self.floors == other.floors && self.elevator == other.elevator
    }
}

impl Eq for Building {}

impl Hash for Building {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.floors.hash(state);
        self.elevator.hash(state);
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Floor {
    items: Vec<Object>
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Object {
    Chip(Element),
    Generator(Element)
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Element {
    Thulium,
    Plutonium,
    Strontium,
    Promethium,
    Ruthenium,
    // Part 2
    Elerium,
    Dilithium
}
