use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let lines: Vec<String> = fs::read_to_string(filename)?.parse::<String>()?.trim().split('\n').map(|x| x.to_string()).collect();

    let mut sol1 = 0;
    let mut vec2 = vec![];
    for line in lines {
        match check(&line) {
            Ok(_) => (),
            Err(ParseErr::Corrupted(score)) => sol1 += score,
            Err(ParseErr::Incomplete(score)) => vec2.push(score)
        }
    }
    vec2.sort_unstable();
    let sol2 = vec2[vec2.len()/2];

    assert!(sol1 == 362271);
    println!("Part 1 = {}", sol1);

    assert!(sol2 == 1698395182);
    println!("Part 2 = {}", sol2);

    Ok(())
}

enum ParseErr {
    Incomplete(u64),
    Corrupted(u64)
} 

fn check(string: &str) -> Result<(),ParseErr>  {
    let mut stack: Vec<char> = vec![];
    for s in string.chars() {
        match s {
            '(' | '[' | '{' | '<' => stack.push(s),
            ')' | ']' | '}' | '>' => {
                if let Some(last) = stack.pop() {
                    match (last, s) {
                        ('(',')') | ('[',']') | ('{','}') | ('<','>') => (),
                        _ => return Err(ParseErr::Corrupted(score1(s)))
                    }
                } else {
                        return Err(ParseErr::Corrupted(score1(s)));
                }
            },
            _ => ()
        }
    }
    if !stack.is_empty() {
        let mut score: u64 = 0;
        while let Some(val) = stack.pop() {
            score *= 5;
            score += score2(val);
        }
        return Err(ParseErr::Incomplete(score));
    }
    Ok(())
}

fn score1(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!()
    }
}

fn score2(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!()
    }
}