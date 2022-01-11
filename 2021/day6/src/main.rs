use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?; //String::new();
    let numbers = line.trim().split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut days = vec![0; 9];
    for n in numbers {
        days[n as usize] += 1;
    }

    let sol1 = fish_num(days.clone(),80);
    assert!(sol1 == 343441);
    println!("Part 1 = {}", sol1);

    let sol2 = fish_num(days.clone(),256);
    assert!(sol2 == 1569108373832);
    println!("Part 2 = {}", sol2);
    Ok(())
}

fn fish_num(mut days: Vec<u64>, total_days: u64) -> u64 {
    for _ in 0..total_days {
        let day_0 = days[0];
        for i in 0..=7 {
            days[i] = days[i+1]
        }
        days[6] += day_0;
        days[8] = day_0;
    }
    days.iter().sum::<u64>()
}