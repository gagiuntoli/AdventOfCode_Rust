use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let sections = line.split("\n\n").collect::<Vec<&str>>();

    let first_sec = sections[0].lines().collect::<Vec<&str>>();
    let instructions = sections[1];

    let mut stacks_initial: Vec<Vec<char>> = vec![vec![]; (first_sec[0].len() + 1) / 4];

    first_sec
        .iter()
        .take(first_sec.len() - 1)
        .rev()
        .for_each(|x| {
            let part = x
                .as_bytes()
                .chunks(4)
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap();

            part.iter().enumerate().for_each(|(i, elem)| {
                let letter = elem.trim().replace("[", "").replace("]", "");
                assert!(letter.len() <= 1);

                match letter.chars().next() {
                    Some(c) => stacks_initial[i].push(c),
                    _ => (),
                }
            });
        });

    let mut stacks = stacks_initial.clone();

    instructions.lines().for_each(|x| {
        let inst = x.split(' ').collect::<Vec<&str>>();
        let qty = inst[1].parse::<usize>().unwrap();
        let from = inst[3].parse::<usize>().unwrap() - 1;
        let to = inst[5].parse::<usize>().unwrap() - 1;

        assert!(to < stacks.len());
        assert!(from < stacks.len());

        (0..qty).for_each(|_| {
            let val: char = stacks[from].pop().unwrap();
            stacks[to].push(val);
        });
    });

    println!(
        "{}",
        stacks.iter().map(|x| x[x.len() - 1]).collect::<String>()
    );

    let mut stacks = stacks_initial.clone();

    instructions.lines().for_each(|x| {
        let inst = x.split(' ').collect::<Vec<&str>>();
        let qty = inst[1].parse::<usize>().unwrap();
        let from = inst[3].parse::<usize>().unwrap() - 1;
        let to = inst[5].parse::<usize>().unwrap() - 1;

        assert!(to < stacks.len());
        assert!(from < stacks.len());

        (0..qty).for_each(|i| {
            let c = stacks[from][stacks[from].len() - qty + i as usize];
            stacks[to].push(c);
        });
        (0..qty).for_each(|_| {
            stacks[from].pop().unwrap();
        });
    });

    println!(
        "{}",
        stacks.iter().map(|x| x[x.len() - 1]).collect::<String>()
    );

    Ok(())
}
