use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let numbers = line.trim().split('\n')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

    println!("{:?}", numbers);

    Ok(())
}
