use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let characters = line.trim().chars().collect::<Vec<char>>();

    for (i, x) in characters.windows(4).enumerate() {
        if !(1..x.len()).any(|i| x[i..].contains(&x[i - 1])) {
            println!("{}", i + 4);
            break;
        }
    }

    for (i, x) in characters.windows(14).enumerate() {
        if !(1..x.len()).any(|i| x[i..].contains(&x[i - 1])) {
            println!("{}", i + 14);
            break;
        }
    }

    Ok(())
}
