use std::fs;
use std::env;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let (part1, part2) = line.trim().split_once("\n\n").unwrap();

    let coordinates = part1.split('\n').map(|line| {
        let (x,y) = line.split_once(',').unwrap();
        return (x.parse::<u32>().unwrap(),y.parse::<u32>().unwrap());
    }).collect::<Vec<(u32,u32)>>();

    let instructions = part2.split('\n').map(|line| {
        let (x,y) = line.split_once('=').unwrap();
        return (x.chars().rev().nth(0).unwrap(),y.parse::<u32>().unwrap());
    }).collect::<Vec<(char,u32)>>();

    let mut mx = coordinates.iter().map(|x| x.0).max().unwrap();
    let mut my = coordinates.iter().map(|x| x.1).max().unwrap();
    
    let mut map = HashMap::<(u32,u32), bool>::new();
    for (x,y) in coordinates {
        map.insert((x,y),true);
    }
    let mut iter_i = instructions.iter();
    let inst_1 = *iter_i.next().unwrap();
    fold(&mut map, inst_1, &mut mx, &mut my);
    let sol1 = map.iter().filter(|(point,_)| point.0 <= mx && point.1 <= my).count();

    for inst in iter_i {
        fold(&mut map, *inst, &mut mx, &mut my);
    }

    println!("Part 1 = {}", sol1);
    println!("Part 2 = ");
    for y in 0..=my {
        let mut string = String::new();
        for x in 0..=mx {
            if map.contains_key(&(x,y)) {
                string.push_str("#")
            } else {
                string.push_str(".")
            }
        }
        println!("{}", string);
    }

    Ok(())
}

fn fold(map: &mut HashMap::<(u32,u32), bool>, inst: (char, u32), mx: &mut u32, my: &mut u32) {

    let mut new_points = vec![];

    if inst.0 == 'x' {
        for (point, _) in map.iter() {
            if point.0 > *mx / 2 && point.0 <= *mx {
                new_points.push(((*mx as i32 - point.0 as i32 + (if *mx % 2 == 1 {-1} else {0})) as u32, point.1));
            }
        }
        *mx = *mx / 2;
    } else {
        for (point, _) in map.iter() {
            if point.1 > *my / 2 && point.1 <= *my {
                new_points.push((point.0, (*my as i32 - point.1 as i32 + (if *my % 2 == 1 {-1} else {0})) as u32));
            }
        }
        *my = *my / 2;
    }

    for point in new_points {
        map.entry(point).or_insert(true);
    }
}