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

    let mut count1 = 0;
    for i in 1..values.len() {
        if values[i] > values[i-1] {
            count1 += 1;
        }
    }
    assert!(count1 == 1583);
    println!("part 1 = {}", count1);

    let mut count2 = 0;
    for i in 1..values.len()-2 {
        if values[i+2] > values[i-1] {
            count2 += 1;
        }
    }
    assert!(count2 == 1627);
    println!("part 2 = {}", count2);
}

/*
part 1 = 1583
part 2 = 1627
*/