use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let numbers = line.trim().split('\n').map(|s| s.to_string()).collect::<Vec<String>>();

    let mut sol1 = 0;
    let mut sol2 = 0;
    for n in numbers {
        let mut range = (0,0);
        let mut letter = "";
        let mut word = "";
        let mut iter = n.split_whitespace();
        if let Some(string) = iter.next() {
            if let Some(split) = string.split_once('-') {
                range = (split.0.parse::<usize>().unwrap(), split.1.parse::<usize>().unwrap());
            }
        }
        if let Some(string) = iter.next() {
            letter = &string[0..1];
        }
        if let Some(string) = iter.next() {
            word = string;
        }

        let reps = word.matches(letter).count();
        if range.0 <= reps && reps <= range.1 {
            sol1 += 1;
        }

        let cond1 = range.0 <= word.len() && &word[(range.0-1)..range.0] == letter;
        let cond2 = range.1 <= word.len() && &word[(range.1-1)..range.1] == letter;
        if (cond1 && !cond2) || (!cond1 && cond2) {
            sol2 += 1;
        }
    }

    assert!(sol1 == 506);
    println!("Part 1 = {}", sol1);

    assert!(sol2 == 443);
    println!("Part 2 = {}", sol2);

    Ok(())
}
