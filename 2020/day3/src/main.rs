use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let map = line.trim().split('\n')
                    .map(|x| x.chars().collect())
                    .collect::<Vec<Vec<char>>>();

    let mut sol1 = 0;
    let mut sol2: u64 = 1;
    for slope in [(1,1), (3,1), (5,1), (7,1), (1, 2)] {
        let trees = trees_found(&map, slope.0, slope.1);
        if slope == (3,1) {
            sol1 = trees;
        }
        sol2 *= trees as u64;
    }

    assert!(sol1 == 151);
    println!("Part 1 = {}", sol1);

    assert!(sol2 == 7540141059);
    println!("Part 2 = {}", sol2);

    Ok(())
}

fn trees_found(map: &Vec<Vec<char>>, right: usize, down: usize) -> u32 {
    let mut count = 0;
    let (mut x, mut y): (usize, usize) = (0, 0);
    while y < map.len() {
        if map[y][x] == '#' {
            count += 1;
        }
        x = (x + right) % map[0].len();
        y += down;
    }
    count
}