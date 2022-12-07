use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let file: String = fs::read_to_string(filename)?.parse()?;

    let mut directories_size = HashMap::<String, u32>::new();
    let mut current_path = Vec::<&str>::new();

    file.lines().for_each(|line| {
        let splitted_line = line.trim().split_whitespace().collect::<Vec<&str>>();

        if splitted_line[0] != "$" {
            if let Ok(size) = splitted_line[0].parse::<u32>() {
                current_path.iter().enumerate().for_each(|(i, _)| {
                    let path_dir = current_path[..=i].join("-");
                    *directories_size.entry(path_dir).or_insert(0) += size;
                });
            }
        } else {
            match splitted_line[..] {
                ["$", "cd", ".."] => {
                    current_path.pop();
                }
                ["$", "cd", dir] => {
                    if dir == "/" {
                        current_path = vec!["/"];
                    } else {
                        current_path.push(dir);
                    }
                }
                ["$", "ls"] => (),
                _ => panic!("invalid command"),
            }
        }
    });

    println!(
        "{}",
        directories_size
            .iter()
            .map(|(_, value)| if *value <= 100000 { *value } else { 0 })
            .sum::<u32>()
    );

    let space_needed = 30000000 - (70000000 - directories_size.get(&"/".to_string()).unwrap());

    println!(
        "{}",
        directories_size
            .into_iter()
            .filter(|(_, value)| *value > space_needed)
            .map(|(_, value)| value)
            .min()
            .unwrap()
    );

    Ok(())
}
