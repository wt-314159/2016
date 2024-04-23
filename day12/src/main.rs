#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    // For Part 1, register 3 needs to be set to 0 also
    let mut registers = [0,0,1,0];
    let mut instructions: Vec<Instruction> = Vec::new();
    #[allow(unused_variables)]
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        let instr = match params[0] {
            "cpy" => Instruction::Cpy(parse_value(params[1]), parse_value(params[2])),
            "inc" => Instruction::Inc(parse_value(params[1])),
            "dec" => Instruction::Dec(parse_value(params[1])),
            "jnz" => Instruction::Jnz(parse_value(params[1]), params[2].parse::<i32>().unwrap()),
            other => panic!("Cannot parse {}", other)
        };
        instructions.push(instr);
    }

    println!("Registers: a, b, c, d:");

    let mut index: i32 = 0;
    let length = instructions.len() as i32;
    while index >= 0 && index < length {
        let instr = &instructions[index as usize];
        match instr {
            Instruction::Cpy(x,y) => { copy(&mut registers, x, y); index += 1},
            Instruction::Inc(x) => { inc(&mut registers, x); index +=1 },
            Instruction::Dec(x) => { dec(&mut registers, x); index += 1 },
            Instruction::Jnz(x, by) => index += jnz(&mut registers, x, *by)
        }
        //println!("[{} {} {} {}]", registers[0], registers[1], registers[2], registers[3]);
    }

    println!("Program has finished, register a has value: {}", registers[0]);
}

fn parse_value(input: &str) -> Value {
    if let Ok(int) = input.parse::<i32>() {
        return Value::Integer(int)
    }
    Value::Register(String::from(input))
}

fn copy(regs: &mut [i32; 4], x: &Value, y: &Value) {
    y.set_value(regs, x.get_value(regs));
}

fn inc(regs: &mut [i32; 4], x: &Value) {
    let curr = x.get_value(regs);
    x.set_value(regs, curr + 1);
}

fn dec(regs: &mut [i32; 4], x: &Value) {
    let curr = x.get_value(regs);
    x.set_value(regs, curr - 1);
}

fn jnz(regs: &mut [i32; 4], x: &Value, by: i32) -> i32 {
    if x.get_value(regs) != 0 {
        return by;
    }
    1
}

enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, i32)
}

enum Value {
    Register(String),
    Integer(i32)
}

impl Value {
    fn get_value(&self, registers: &[i32; 4]) -> i32 {
        match self {
            Value::Integer(int) => *int,
            Value::Register(reg_name) => {
                match reg_name.as_str() {
                    "a" => registers[0],
                    "b" => registers[1],
                    "c" => registers[2],
                    "d" => registers[3],
                    other => panic!("No register with name {}", other)
                }
            }
        }
    }

    fn set_value(&self, registers: &mut [i32; 4], value: i32) {
        match self {
            Value::Register(reg_name) => {
                match reg_name.as_str() {
                    "a" => registers[0] = value,
                    "b" => registers[1] = value,
                    "c" => registers[2] = value,
                    "d" => registers[3] = value,
                    other => panic!("No register with name {}", other)
                }
            },
            Value::Integer(_) => panic!("Cannot set value")
        }
    }
}