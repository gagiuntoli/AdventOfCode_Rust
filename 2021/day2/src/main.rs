use std::fs::File;
use std::io::{self, BufRead};
use std::env;

struct Instruction {
    direction : String,
    value: u64,
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();

    let mut values : Vec<Instruction> = Vec::new();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(ip) = line {
            let instruction : Vec<&str> = ip.split(' ').collect();
            values.push(Instruction {
                direction: instruction[0].to_string(),
                value: instruction[1].parse::<u64>().unwrap()
            });
        }
    }

    let mut horizontal = 0;
    let mut vertical = 0;
    for v in &values {
        match v.direction.as_ref() {
            "forward" => horizontal += v.value,
            "up" => vertical -= v.value,
            "down" => vertical += v.value,
            _ => println!("Direction not recognized: {}", v.direction)
        } 
    }
    println!("part 1 = {}", horizontal * vertical);

    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;
    for v in &values {
        match v.direction.as_ref() {
            "forward" => {horizontal += v.value; vertical += aim * v.value},
            "up" => aim -= v.value,
            "down" => aim += v.value,
            _ => println!("Direction not recognized: {}", v.direction)
        } 
    }
    println!("part 2 = {}", horizontal * vertical);
}