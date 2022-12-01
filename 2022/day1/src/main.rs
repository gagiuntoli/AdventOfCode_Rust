use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let sol1 = line
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!("{:?}", sol1);

    let mut sol2 = line
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|num| num.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    sol2.sort();

    let sol2 = &sol2[sol2.len() - 3..].into_iter().sum::<u32>();

    println!("{:?}", sol2);

    Ok(())
}
