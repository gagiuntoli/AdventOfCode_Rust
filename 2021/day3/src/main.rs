use std::fs::File;
use std::io::{self, BufRead};
use std::env;

const WIDTH : usize = 12;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();
    let values = io::BufReader::new(file)
        .lines()
        .map(|l| usize::from_str_radix(l.unwrap().as_str(), 2).unwrap())
        .collect::<Vec<usize>>();

    let length = values.len();

    let count_vec = values
        .clone()
        .iter()
        .fold(vec![0; WIDTH], |mut count, bits| {
            for i in 0..WIDTH {
                count[i] += if bits & (1 << i) != 0 {1} else {0};
            }
            count
        });

    let gamma = count_vec
        .iter()
        .enumerate()
        .map(|(i,c)| ((c > &(length/2)) as u32) << i)
        .sum::<u32>();

    let epsilon = !gamma & ((1 << WIDTH) - 1);

    assert!(gamma * epsilon == 3901196);
    println!("part 1 = {}", gamma * epsilon);

    let mut oxr = values.clone();
    let mut co2 = values.clone();

    for bit in (0..WIDTH).rev() {

        let count_oxr = oxr
            .iter()
            .fold(vec![0; WIDTH], |mut count, bits| {
                for i in 0..WIDTH {
                    count[i] += if bits & (1 << i) != 0 {1} else {0};
                }
                count
            });

        if count_oxr[bit] * 2 >= oxr.len() {
            oxr.retain(|bits| ((bits & (1 << bit)) != 0));
        } else {
            oxr.retain(|bits| ((bits & (1 << bit)) == 0));
        }
        if oxr.len() == 1 {
            break;
        }
    }

    for bit in (0..WIDTH).rev() {

        let count_co2 = co2
            .iter()
            .fold(vec![0; WIDTH], |mut count, bits| {
                for i in 0..WIDTH {
                    count[i] += if bits & (1 << i) != 0 {1} else {0};
                }
                count
            });

        if count_co2[bit] * 2 < co2.len() {
            co2.retain(|bits| ((bits & (1 << bit)) != 0));
        } else {
            co2.retain(|bits| ((bits & (1 << bit)) == 0));
        }
        if co2.len() == 1 {
            break;
        }
    }
    assert!(oxr[0] * co2[0] == 4412188);
    println!("part 2 = {}", oxr[0] * co2[0]);
}