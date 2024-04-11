#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max, rc::Rc, cell::{RefCell, RefMut}};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut outputs: HashMap<usize, Rc<RefCell<Output>>> = HashMap::new();
    let mut bots: HashMap<usize, Rc<RefCell<Bot>>> = HashMap::new();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        if params.len() == 6 {
            let bot_id = params[5].parse::<usize>().unwrap();
            let chip = params[1].parse::<usize>().unwrap();
            let entry = bots.entry(bot_id).or_insert(Rc::new(RefCell::new(Bot::new(bot_id))));
            let bot = bots.get(&bot_id).unwrap();
            let cloned = Rc::clone(bot);
            let mut borrowed = (*cloned).borrow_mut();
            borrowed.add_chip(chip);
        }
        else {
            let bot_one = params[1].parse::<usize>().unwrap();
            let entry1 = get_entry_if_present(&bots, &bot_one);
            let mut bot = Bot::new(bot_one);

            if let Some(bot_entry) = entry1.clone() {
                let mut bot = bot_entry.borrow_mut();
                add_receiving_bots(&mut bots, &mut outputs, &mut bot, params[5], params[6], params[10], params[11]);
                continue;
            }
            add_receiving_bots(&mut bots, &mut outputs,&mut bot, params[5], params[6], params[10], params[11]);
            bots.insert(bot_one, Rc::new(RefCell::new(bot)));
        }
    }

    loop {
        let both_chip_bots: Vec<(&usize, &Rc<RefCell<Bot>>)> = bots.iter().filter(|(_, bot)| bot.borrow().has_both_chips()).collect();
        if both_chip_bots.len() == 0 {
            break;
        }
        for bot in both_chip_bots {
            let result = bot.1.borrow_mut().give_chips();
            if result {
                println!("Found bot!");
            }
        }
    }

    let out_0 = outputs.get(&0).unwrap().borrow().chip.unwrap();
    let out_1 = outputs.get(&1).unwrap().borrow().chip.unwrap();
    let out_2 = outputs.get(&2).unwrap().borrow().chip.unwrap();
    let multiplied = out_0 * out_1 * out_2;
    println!("Outputs 0, 1, and 2 have chips {}, {} and {} respectively, multiplied that gives {}", out_0, out_1, out_2, multiplied);
}

fn get_entry_if_present(bots: &HashMap<usize, Rc<RefCell<Bot>>>, id: &usize) -> Option<Rc<RefCell<Bot>>> {
    bots.get(id).cloned()
}

fn add_receiving_bots(
    bots: &mut HashMap<usize, Rc<RefCell<Bot>>>, 
    outputs: &mut HashMap<usize, Rc<RefCell<Output>>>,
    source_bot: &mut Bot, 
    param5: &str, 
    param6: &str, 
    param10: &str, 
    param11: &str) {
    if param5 == "bot" {
        let bot_two = param6.parse::<usize>().unwrap();
        if let Some(entry2) = bots.get(&bot_two) {
            source_bot.give_low_bot = Some(entry2.clone());
        }
        else {
            let entry2 = Rc::new(RefCell::new(Bot::new(bot_two)));
            source_bot.give_low_bot = Some(entry2.clone());
            bots.insert(bot_two, entry2);
        }
    }
    else if param5 == "output" {
        let out_two = param6.parse::<usize>().unwrap();
        if let Some(entry2) = outputs.get(&out_two) {
            source_bot.low_output = Some(entry2.clone());
        }
        else {
            let entry2 = Rc::new(RefCell::new(Output { id: out_two, chip: None}));
            source_bot.low_output = Some(entry2.clone());
            outputs.insert(out_two, entry2);
        }
    }
    if param10 == "bot" {
        let bot_thr = param11.parse::<usize>().unwrap();
        if let Some(entry3) = bots.get(&bot_thr) {
            source_bot.give_high_bot = Some(entry3.clone());
        }
        else {
            let entry3 = Rc::new(RefCell::new(Bot::new(bot_thr)));
            source_bot.give_high_bot = Some(entry3.clone());
            bots.insert(bot_thr, entry3);
        }
    }
    else if param10 == "output" {
        let out_thr = param11.parse::<usize>().unwrap();
        if let Some(entry3) = outputs.get(&out_thr) {
            source_bot.low_output = Some(entry3.clone());
        }
        else {
            let entry3 = Rc::new(RefCell::new(Output { id: out_thr, chip: None}));
            source_bot.low_output = Some(entry3.clone());
            outputs.insert(out_thr, entry3);
        }
    }
}

struct Bot {
    id: usize,
    chip_1: Option<usize>,
    chip_2: Option<usize>,
    give_low_bot: Option<Rc<RefCell<Bot>>>,
    give_high_bot: Option<Rc<RefCell<Bot>>>,
    low_output: Option<Rc<RefCell<Output>>>,
    high_output: Option<Rc<RefCell<Output>>>
}

struct Output {
    id: usize,
    chip: Option<usize>
}

impl Bot {
    fn get_chip(&self, low: bool) -> usize {
        if let (Some(one), Some(two)) = (self.chip_1, self.chip_2) {
            let min = min(one, two);
            let max = max(one, two);
            if min == 17 && max == 61 {
                println!("Bot ID: {}", self.id);
            }
            match low {
                true => min,
                false => max
            }
        }
        else {
            panic!("One of chips was None");
        }
    }

    fn new(id: usize) -> Bot {
        Bot { id, chip_1: None, chip_2: None, give_low_bot: None, give_high_bot: None, low_output: None, high_output: None }
    }

    fn add_chip(&mut self, chip: usize) {
        if let None = self.chip_1 {
            self.chip_1 = Some(chip);
        }
        else if let None = self.chip_2 {
            self.chip_2 = Some(chip);
        }
        else {
            panic!("Both chips already full! Bot ID: '{}', Chip 1: {}, Chip 2: {}, new chip: {}", self.id, self.chip_1.unwrap(), self.chip_2.unwrap(), chip);
        }
    }

    fn give_chips(&mut self) -> bool {
        if self.chip_1 == None || self.chip_2 == None {
            panic!("One of chips is None");
        }
        let one = self.chip_1.unwrap();
        let two = self.chip_2.unwrap();
        let min = min(one, two);
        let max = max(one, two);
        let first_low = one < two;
        if min == 17 && max == 61 {
            println!("Bot ID: {}", self.id);
            // return true;
        }
        if let Some(low_bot) = &self.give_low_bot {
            low_bot.borrow_mut().add_chip(min);
            if first_low {
                self.chip_1 = None;
            }
            else {
                self.chip_2 = None;
            }
        } 
        else if let Some(low_output) = &self.low_output {
            low_output.borrow_mut().chip = Some(min);
            if first_low {
                self.chip_1 = None;
            }
            else {
                self.chip_2 = None;
            }
        }
        if let Some(high_bot) = &self.give_high_bot {
            high_bot.borrow_mut().add_chip(max);
            if first_low {
                self.chip_2 = None;
            }
            else {
                self.chip_1 = None;
            }
        }
        else if let Some(high_output) = &self.high_output {
            high_output.borrow_mut().chip = Some(max);
            if first_low {
                self.chip_2 = None;
            }
            else {
                self.chip_1 = None;
            }
        }
        false
    }

    fn has_both_chips(&self) -> bool {
        self.chip_1 != None && self.chip_2 != None
    }
}
