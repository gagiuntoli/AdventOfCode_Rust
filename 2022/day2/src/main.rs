use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;

    let sol1 = line
        .trim()
        .split('\n')
        .map(|x| {
            let game = x.split(' ').collect::<Vec<&str>>();
            return match game[..] {
                ["A", "X"] => 1 + 3,
                ["A", "Y"] => 2 + 6,
                ["A", "Z"] => 3 + 0,
                ["B", "X"] => 1 + 0,
                ["B", "Y"] => 2 + 3,
                ["B", "Z"] => 3 + 6,
                ["C", "X"] => 1 + 6,
                ["C", "Y"] => 2 + 0,
                ["C", "Z"] => 3 + 3,
                _ => panic!("not a match"),
            };
        })
        .sum::<usize>();

    println!("{:?}", sol1);

    let sol2 = line
        .trim()
        .split('\n')
        .map(|x| {
            let game = x.split(' ').collect::<Vec<&str>>();
            return match game[..] {
                ["A", "X"] => 3 + 0,
                ["A", "Y"] => 1 + 3,
                ["A", "Z"] => 2 + 6,
                ["B", "X"] => 1 + 0,
                ["B", "Y"] => 2 + 3,
                ["B", "Z"] => 3 + 6,
                ["C", "X"] => 2 + 0,
                ["C", "Y"] => 3 + 3,
                ["C", "Z"] => 1 + 6,
                _ => panic!("not a match"),
            };
        })
        .sum::<usize>();

    println!("{:?}", sol2);

    Ok(())
}
