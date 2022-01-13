use std::fs;
use std::env;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let lines: Vec<String> = fs::read_to_string(filename)?.split("\n").map(|s| s.to_string()).collect();

    let mut sol1 = 0;
    let mut sol2 = 0;
    for line in lines {
        if let Some((part1, part2)) = line.split_once(" | ") {
            let mut dig1: Vec<String> = part1.split(" ").map(|x| sort_str(x)).collect();
            let mut dig2: Vec<String> = part2.split(" ").map(|x| sort_str(x)).collect();
            sol1 += dig2
                .iter()
                .filter(|x| x.len()==2 || x.len()==3 || x.len()==4 || x.len()==7)
                .count();
            dig1.append(&mut dig2);
            sol2 += convert(&dig1);
        }
    }

    assert!(sol1 == 514);
    println!("Part 1 = {}", sol1);
    assert!(sol2 == 1012272);
    println!("Part 2 = {}", sol2);

    Ok(())
}

fn sort_str(str_: &str) -> String {
    let wordy = String::from(str_);
    let mut chars: Vec<char> = wordy.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(chars)
}

fn convert(strings: &Vec<String>) -> u32 {

    let mut map: HashMap<String, u8> = HashMap::new();
    let one = strings.iter().find(|x| x.len()==2).unwrap();
    let four = strings.iter().find(|x| x.len()==4).unwrap();

    for s in strings {
        let num = match s.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            len => {
                match (len,
                        one.chars().map(|x| s.matches(x).count()).sum(),
                        four.chars().map(|x| s.matches(x).count()).sum()) {
                    (6, 2, 4) => 9,
                    (6, 1, 3) => 6,
                    (6, 2, 3) => 0,
                    (5, 1, 2) => 2,
                    (5, 2, 3) => 3,
                    (5, 1, 3) => 5,
                    (_, _, _) => unreachable!()
                }
            }
        };
        map.entry(s.to_string()).or_insert(num);
    }

    (1..=4).rev().map(|i| (map[&strings[strings.len()-i]] as u32) * 10_u32.pow((i-1) as u32)).sum()
}