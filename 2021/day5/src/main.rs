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

    let count = count_intersections(&segments, false);
    assert!(count == 5294);
    println!("Part 1 = {}", count);

    let count = count_intersections(&segments, true);
    assert!(count == 21698);
    println!("Part 2 = {}", count);
}

fn count_intersections(segments: &Vec<Vec<i32>>, diagonals: bool) -> u32 {

    let mut map: HashMap<(i32,i32),u32> = HashMap::new();
    for s in segments {

        let x1 = s[0];
        let y1 = s[1];
        let x2 = s[2];
        let y2 = s[3];
        
        let range_x: Vec<i32> =
        if x2 > x1 {
            (x1..=x2).collect()
        } else if x1 > x2 {
            (x2..=x1).rev().collect()
        } else {
            vec![x1; ((y2-y1).abs() + 1) as usize]
        };
        let range_y: Vec<i32> =
        if y2 > y1 {
            (y1..=y2).collect()
        } else if y1 > y2 {
            (y2..=y1).rev().collect()
        } else {
            vec![y1; ((x2-x1).abs() + 1) as usize]
        };
        if diagonals || (!diagonals && (x1 == x2 || y1 == y2)) {
            for (x,y) in range_x.iter().zip(range_y) {
                let count = map.entry((*x,y)).or_insert(0);
                *count += 1;
            }
        }
    }

    return map.values().filter(|&v| *v > 1).count() as u32;
}