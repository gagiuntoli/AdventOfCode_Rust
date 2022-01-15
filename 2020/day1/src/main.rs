use std::fs;
use std::env;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let numbers = line.trim().split('\n')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

    let mut map = HashMap::<u32,bool>::new();
    let mut sol1 = 0;
    for n in &numbers {
        map.insert(*n, true);
        let val = 2020 - n;
        if map.contains_key(&val) {
            sol1 = val * n;
            break;
        }
    }

    assert!(sol1 == 290784);
    println!("Part 1 = {}", sol1);

    let mut map = HashMap::<u32,u32>::new();
    for i in 0..numbers.len()-1 {
        for j in (i+1)..numbers.len() {
            map.insert(numbers[i] + numbers[j], numbers[i] * numbers[j]);
        }
    }
    let mut sol2 = 0;
    for n in &numbers {
        let val = 2020 - n;
        if map.contains_key(&val) {
            sol2 = map[&val] * n;
            break;
        }
    }
    assert!(sol2 == 177337980);
    println!("Part 2 = {}", sol2);

    Ok(())
}
