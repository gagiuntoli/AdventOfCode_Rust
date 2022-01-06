use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut segments: Vec<Vec<i32>> = vec![];

    for line in lines {
        if let Some((a,b)) = line.unwrap().split_once(" -> ") {
            if let Some((x1,y1)) = a.split_once(",") {
                if let Some((x2,y2)) = b.split_once(",") {
                    segments.push([x1, y1, x2, y2].into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>());
                }
            }
        }
    }

    let mut map: HashMap<(i32,i32),u32> = HashMap::new();
    for s in &segments {
        let x1 = s[0];
        let y1 = s[1];
        let x2 = s[2];
        let y2 = s[3];
        
        let range_x = if x2 > x1 {x1..=x2} else {x2..=x1};
        let range_y = if y2 > y1 {y1..=y2} else {y2..=y1};
        if x1 == x2 {
            for y in range_y {
                let count = map.entry((x1,y)).or_insert(0);
                *count += 1;
            }
        } else if y1 == y2 {
            for x in range_x {
                let count = map.entry((x,y1)).or_insert(0);
                *count += 1;
            }
        }
    }
    let count = map
        .values()
        .filter(|&v| *v > 1)
        .count();

    assert!(count == 5294);
    println!("Part 1 = {}", count);

    let mut map: HashMap<(i32,i32),u32> = HashMap::new();
    for s in &segments {
        let x1 = s[0];
        let y1 = s[1];
        let x2 = s[2];
        let y2 = s[3];
        
        let range_x: Vec<i32> = if x2 > x1 {(x1..=x2).collect()} else {(x2..=x1).rev().collect()};
        let range_y: Vec<i32> = if y2 > y1 {(y1..=y2).collect()} else {(y2..=y1).rev().collect()};
        if x1 == x2 {
            for y in range_y {
                let count = map.entry((x1,y)).or_insert(0);
                *count += 1;
            }
        } else if y1 == y2 {
            for x in range_x {
                let count = map.entry((x,y1)).or_insert(0);
                *count += 1;
            }
        } else {
            for (x,y) in range_x.iter().zip(range_y) {
                let count = map.entry((*x,y)).or_insert(0);
                *count += 1;
            }
        }
    }

    let count = map
        .values()
        .filter(|&v| *v > 1)
        .count();

    assert!(count == 21698);
    println!("Part 2 = {}", count);
}