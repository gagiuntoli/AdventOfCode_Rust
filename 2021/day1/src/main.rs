use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut values : Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines {
            if let Ok(ip) = line {
                values.push(ip.parse::<u32>().unwrap());
            }
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}