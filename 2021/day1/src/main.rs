use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();

    let mut values : Vec<u32> = Vec::new();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(ip) = line {
            values.push(ip.parse::<u32>().unwrap());
        }
    }

    let mut count1 = 0;
    for i in 1..values.len() {
        if values[i] > values[i-1] {
            count1 += 1;
        }
    }
    let mut count2 = 0;
    for i in 1..values.len()-2 {
        if values[i+2] > values[i-1] {
            count2 += 1;
        }
    }
    println!("part 1 = {}", count1);
    println!("part 2 = {}", count2);
}