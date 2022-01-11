use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?; //String::new();
    let numbers = line.trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let sol1 = (min..=max).map(|i| numbers.iter().fold(0, |sum, x| sum + (x-i).abs())).min().unwrap();
    println!("Part 1 = {}", sol1);
    assert!(sol1 == 355989);

    let sol2 = (min..=max).map(|i| numbers.iter().fold(0, |sum, x| sum + (x-i).abs() * ((x-i).abs()+1) / 2)).min().unwrap();
    assert!(sol2 == 102245489);
    println!("Part 2 = {}", sol2);
    Ok(())
}