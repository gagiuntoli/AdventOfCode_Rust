use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut bingos: Vec<Vec<Vec<i32>>> = Vec::new();

    let mut numbers = vec![];
    let mut bingo = vec![];
    for (i,line) in lines.enumerate() {
        if i == 0 {
            numbers = line.as_ref().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        } else if i > 1 {
            if line.as_ref().unwrap().is_empty() {
                bingos.push(bingo.clone());
                bingo = vec![];
            } else {
                bingo.push(line.as_ref().unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>().clone());
            }
        }
    }

    let mut bingo_last_num = 0;
    let mut bingo_num = 0;
    'outer: for num in &numbers {
        for bi in 0..bingos.len() {
            bingos[bi].iter_mut().for_each(|row| row.iter_mut().for_each(|x| *x = if *x == *num {-1} else {*x}));
            if check_bingo(&bingos[bi]) {
                bingo_last_num = *num;
                bingo_num = bi;
                break 'outer;
            }
        }
    }

    let sum_not_marked = bingos[bingo_num].clone().into_iter().map(|row| row.into_iter().map(|x| if x >= 0 {x} else {0}).sum::<i32>()).sum::<i32>();
    assert!(bingo_last_num * sum_not_marked == 71708);
    println!("Part 1 = {}", bingo_last_num * sum_not_marked);

    let mut bingo_last_num = 0;
    let mut bingo_num = 0;
    let mut bingos_solved = vec![0; bingos.len()];
    for num in &numbers {
        for bi in 0..bingos.len() {
            if bingos_solved[bi] == 0 {
                bingos[bi].iter_mut().for_each(|row| row.iter_mut().for_each(|x| *x = if *x == *num {-1} else {*x}));
                if check_bingo(&bingos[bi]) {
                    bingo_last_num = *num;
                    bingo_num = bi;
                    bingos_solved[bi] = 1;
                }
            }
        }
    }

    let sum_not_marked = bingos[bingo_num].clone().into_iter().map(|row| row.into_iter().map(|x| if x >= 0 {x} else {0}).sum::<i32>()).sum::<i32>();
    assert!(bingo_last_num * sum_not_marked == 34726);
    println!("Part 2 = {}", bingo_last_num * sum_not_marked);
}

fn check_bingo(bingo: &Vec<Vec<i32>>) -> bool {
    for i in 0..5 {
        let mut is_bingo = true;
        for j in 0..5 {
            if bingo[i][j] != -1 {
                is_bingo = false;
                break;
            }
        }
        if is_bingo {
            return true;
        }
    }
    for j in 0..5 {
        let mut is_bingo = true;
        for i in 0..5 {
            if bingo[i][j] != -1 {
                is_bingo = false;
                break;
            }
        }
        if is_bingo {
            return true;
        }
    }
    return false;
}