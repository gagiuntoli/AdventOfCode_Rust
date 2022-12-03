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
            let str1 = &x[..x.len() / 2];
            let str2 = &x[x.len() / 2..];
            let reps = str1
                .chars()
                .filter(|&c| str2.contains(c))
                .collect::<Vec<char>>();

            assert!(reps.len() > 0);
            assert!(reps.iter().all(|&item| item == reps[0]));

            match reps[0] as u32 {
                97..=122 => reps[0] as u32 - 96,
                65..=90 => reps[0] as u32 - 65 + 27,
                _ => panic!("invalid char"),
            }
        })
        .sum::<u32>();

    println!("{:?}", sol1);

    let binding = line.trim().split('\n').collect::<Vec<&str>>();

    let sol2 = binding
        .chunks(3)
        .map(|x| {
            let reps = x[0]
                .chars()
                .filter(|&c| x[1].contains(c) && x[2].contains(c))
                .collect::<Vec<char>>();

            assert!(reps.len() > 0);
            assert!(reps.iter().all(|&item| item == reps[0]));

            match reps[0] as u32 {
                97..=122 => reps[0] as u32 - 96,
                65..=90 => reps[0] as u32 - 65 + 27,
                _ => panic!("invalid char"),
            }
        })
        .sum::<u32>();

    println!("{:?}", sol2);

    Ok(())
}
