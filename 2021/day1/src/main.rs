use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let values = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<u32>>();

    let count1 = values.iter().enumerate().filter(|&(i,_)| i != 0 && values[i] > values[i-1]).count();
    //assert!(count1 == 1583);
    println!("part 1 = {}", count1);

    let count2 = values.iter().enumerate().filter(|&(i,_)| i != 0 && i < values.len()-2 && values[i+2] > values[i-1]).count();
    //assert!(count2 == 1627);
    println!("part 2 = {}", count2);
}