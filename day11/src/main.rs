#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let floor_one: Vec<Object> = Vec::new();
    let floor_two: Vec<Object> = Vec::new();
    let floor_three: Vec<Object> = Vec::new();
    let floor_four: Vec<Object> = Vec::new();
    let mut floors = vec![floor_one, floor_two, floor_three, floor_four];

    // manually entering, just quicker to type it out than to 
    // parse the input, the input shouldn't change so no reason
    // to parse it
    floors[0].push(Object::Elevator);
    floors[0].push(Object::Generator(Element::Thulium));
    floors[0].push(Object::Chip(Element::Thulium));
    floors[0].push(Object::Generator(Element::Plutonium));
    floors[0].push(Object::Generator(Element::Strontium));
    floors[1].push(Object::Chip(Element::Plutonium));
    floors[1].push(Object::Chip(Element::Strontium));
    floors[2].push(Object::Generator(Element::Promethium));
    floors[2].push(Object::Chip(Element::Promethium));
    floors[2].push(Object::Generator(Element::Ruthenium));
    floors[2].push(Object::Chip(Element::Ruthenium));

    print_floors(&floors);

    let mut elevator_floor: usize = 0;
    let mut next_floors: Vec<usize> = vec![1];

    let f2_unshielded = chips_without_gens(&floors[0]);

    loop {
        for nx_fl in &next_floors {
            let next_floor = &floors[*nx_fl];
            // find chips on that floor without shielding
            let unshielded_chips = chips_without_gens(next_floor);
            // can take at most 2 generators with us, so if more than 2
            // unshielded chips on next floor, we can't go there
            if unshielded_chips.len() > 2 {
                continue;
            }
        }
        break;
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

fn print_floors(floors: &Vec<Vec<Object>>) {
    for i in (0..4).rev() {
        let floor = &floors[i];
        println!("F{} {} {} {} {} {} {} {} {} {} {} {}", 
        i + 1,
        get_obj_str_or_empty(floor, Object::Elevator),
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
            Object::Generator(Element::Ruthenium) => String::from("RuG"),
            Object::Elevator => String::from("E  ")
        }
    } 
    String::from(".  ")
}

#[derive(PartialEq)]
enum Object {
    Chip(Element),
    Generator(Element),
    Elevator
}

#[derive(PartialEq)]
enum Element {
    Thulium,
    Plutonium,
    Strontium,
    Promethium,
    Ruthenium
}
