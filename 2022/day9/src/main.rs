use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::str::Lines;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let file: String = fs::read_to_string(filename)?.parse()?;

    println!("{}", solution(file.lines(), 2));
    println!("{}", solution(file.lines(), 10));

    Ok(())
}

fn solution(instructions: Lines, number_of_knots: usize) -> usize {
    let mut visited = HashMap::<(i32, i32), bool>::new();

    let mut position_knots = vec![(0i32, 0i32); number_of_knots];
    let last = number_of_knots - 1;
    visited.insert(position_knots[last], true);

    instructions.for_each(|line| {
        let line_splitted = line.trim().split(' ').collect::<Vec<&str>>();
        match line_splitted[..] {
            ["D", amount] => (0..amount.parse::<i32>().unwrap()).for_each(|_| {
                position_knots[0].1 -= 1;
                update_positions(&mut position_knots);
                visited.insert(position_knots[last], true);
            }),
            ["U", amount] => (0..amount.parse::<i32>().unwrap()).for_each(|_| {
                position_knots[0].1 += 1;
                update_positions(&mut position_knots);
                visited.insert(position_knots[last], true);
            }),
            ["L", amount] => (0..amount.parse::<i32>().unwrap()).for_each(|_| {
                position_knots[0].0 -= 1;
                update_positions(&mut position_knots);
                visited.insert(position_knots[last], true);
            }),
            ["R", amount] => (0..amount.parse::<i32>().unwrap()).for_each(|_| {
                position_knots[0].0 += 1;
                update_positions(&mut position_knots);
                visited.insert(position_knots[last], true);
            }),
            _ => println!("not matched"),
        };
    });

    visited.keys().len()
}

fn update_positions(position_knots: &mut Vec<(i32, i32)>) {
    for i in 1..position_knots.len() {
        update_position_t(position_knots[i - 1], &mut position_knots[i]);
    }
}

fn update_position_t(position_h: (i32, i32), position_t: &mut (i32, i32)) {
    if position_h.0 == position_t.0 {
        let dy = position_h.1 - position_t.1;
        if dy > 1 {
            position_t.1 += 1;
        } else if dy < -1 {
            position_t.1 -= 1;
        }
    } else if position_h.1 == position_t.1 {
        let dx = position_h.0 - position_t.0;
        if dx > 1 {
            position_t.0 += 1;
        } else if dx < -1 {
            position_t.0 -= 1;
        }
    } else if (position_h.0 - position_t.0).abs() == 1 {
        if position_h.1 > position_t.1 + 1 {
            position_t.0 = position_h.0;
            position_t.1 = position_h.1 - 1;
        }
        if position_h.1 < position_t.1 - 1 {
            position_t.0 = position_h.0;
            position_t.1 = position_h.1 + 1;
        }
    } else if (position_h.1 - position_t.1).abs() == 1 {
        if position_h.0 > position_t.0 + 1 {
            position_t.0 = position_h.0 - 1;
            position_t.1 = position_h.1;
        }
        if position_h.0 < position_t.0 - 1 {
            position_t.0 = position_h.0 + 1;
            position_t.1 = position_h.1;
        }
    } else if (position_h.0 - position_t.0).abs() == 2 && (position_h.1 - position_t.1).abs() == 2 {
        if position_h.0 == position_t.0 + 2 {
            position_t.0 = position_h.0 - 1;
        } else {
            position_t.0 = position_h.0 + 1;
        }
        if position_h.1 == position_t.1 + 2 {
            position_t.1 = position_h.1 - 1;
        } else {
            position_t.1 = position_h.1 + 1;
        }
    }
}
