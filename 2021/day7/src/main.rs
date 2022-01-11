use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let mut buffer = io::BufReader::new(file);
    let mut first_line = String::new();
    let _ = buffer.read_line(&mut first_line);
    let numbers = first_line.trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let sol1 = (min..=max).map(|i| numbers.iter().fold(0, |sum, x| sum + (x-i).abs())).min().unwrap();
    println!("Part 1 = {}", sol1);
    assert!(sol1 == 355989);

    let sol2 = (min..=max).map(|i| numbers.iter().fold(0, |sum, x| sum + (x-i).abs() * ((x-i).abs()+1) / 2)).min().unwrap();
    assert!(sol2 == 102245489);
    println!("Part 2 = {}", sol2);
}