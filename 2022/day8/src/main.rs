use std::cmp::min;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let file: String = fs::read_to_string(filename)?.parse()?;
    let forest = file
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let sol = forest[1..forest.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row[1..row.len() - 1]
                .iter()
                .enumerate()
                .map(|(j, &tree_height)| {
                    let i = i + 1;
                    let j = j + 1;
                    return if forest[0..i].iter().all(|row| row[j] < tree_height)
                        || forest[i + 1..].iter().all(|row| row[j] < tree_height)
                        || forest[i][j + 1..].iter().all(|&val| val < tree_height)
                        || forest[i][..j].iter().all(|&val| val < tree_height)
                    {
                        1
                    } else {
                        0
                    };
                })
                .sum::<u32>()
        })
        .sum::<u32>()
        + (forest.len() as u32 * 2)
        + (forest[0].len() as u32 - 2) * 2;

    println!("{:?}", sol);

    let sol = forest[1..forest.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row[1..row.len() - 1]
                .iter()
                .enumerate()
                .map(|(j, &tree_height)| {
                    let i = i + 1;
                    let j = j + 1;
                    min(
                        forest[0..i].len(),
                        forest[0..i]
                            .iter()
                            .rev()
                            .take_while(|row| row[j] < tree_height)
                            .count()
                            + 1,
                    ) * min(
                        forest[i + 1..].len(),
                        forest[i + 1..]
                            .iter()
                            .take_while(|row| row[j] < tree_height)
                            .count()
                            + 1,
                    ) * min(
                        forest[i][..j].len(),
                        forest[i][..j]
                            .iter()
                            .rev()
                            .take_while(|&val| *val < tree_height)
                            .count()
                            + 1,
                    ) * min(
                        forest[i][j + 1..].len(),
                        forest[i][j + 1..]
                            .iter()
                            .take_while(|&val| *val < tree_height)
                            .count()
                            + 1,
                    )
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{:?}", sol);

    Ok(())
}
