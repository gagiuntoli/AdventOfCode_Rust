use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Inst {
    Addx(i32),
    Noop,
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let instructions = line
        .trim()
        .lines()
        .map(|x| {
            let splitted_line = x.split(' ').collect::<Vec<&str>>();
            match splitted_line[..] {
                ["addx", val] => Inst::Addx(val.parse::<i32>().unwrap()),
                ["noop"] => Inst::Noop,
                _ => panic!("invalid instruction"),
            }
        })
        .collect::<Vec<Inst>>();

    let register_values = run_instructions(&instructions);
    let special_cycles = [20usize, 60, 100, 140, 180, 220];
    println!(
        "{}",
        special_cycles.iter().fold(0, |acc, cycle| acc
            + (*cycle as i32) * register_values[*cycle - 1])
    );

    const width: usize = 40;
    const hight: usize = 6;
    let mut screen = vec!['.'; width * hight];

    register_values
        .iter()
        .enumerate()
        .for_each(|(cycle, sprite_pos)| {
            if cycle % width >= (*sprite_pos as usize - 1)
                && cycle % width <= (*sprite_pos as usize + 1)
            {
                screen[cycle] = '#';
            }
        });

    screen
        .chunks(width)
        .for_each(|row| println!("{:?}", row.iter().collect::<String>()));

    Ok(())
}

fn run_instructions(instructions: &Vec<Inst>) -> Vec<i32> {
    let mut register_values = vec![1, 1];
    instructions.iter().for_each(|inst| {
        let last_val = register_values[register_values.len() - 1];
        match inst {
            Inst::Addx(val) => {
                register_values.push(last_val + val);
                register_values.push(last_val + val);
            }
            Inst::Noop => register_values.push(last_val),
        };
    });
    register_values
}
