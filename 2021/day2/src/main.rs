use std::fs::File;
use std::io::{self, BufRead};
use std::env;

struct Inst {
    dir: String,
    val: u64,
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();

    let values = io::BufReader::new(file)
        .lines()
        .map(|line| {
            match line.unwrap().split_once(' ').unwrap() {
                (a,b) => Inst {dir: a.to_string(), val: b.parse().unwrap()}
            }
        })
        .collect::<Vec<Inst>>();

    let mut horizontal = 0;
    let mut vertical = 0;
    for v in &values {
        match v.dir.as_ref() {
            "forward" => horizontal += v.val,
            "up" => vertical -= v.val,
            "down" => vertical += v.val,
            _ => unreachable!(),
        } 
    }
    assert!(horizontal * vertical == 2039256);
    println!("part 1 = {}", horizontal * vertical);

    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;
    for v in &values {
        match v.dir.as_ref() {
            "forward" => {horizontal += v.val; vertical += aim * v.val},
            "up" => aim -= v.val,
            "down" => aim += v.val,
            _ => unreachable!()
        } 
    }
    assert!(horizontal * vertical == 1856459736);
    println!("part 2 = {}", horizontal * vertical);
}