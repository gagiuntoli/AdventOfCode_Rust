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
            let words = x.split(',').collect::<Vec<&str>>();
            match words[..] {
                [str1, str2] => {
                    let range1 = str1
                        .split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();

                    let range2 = str2
                        .split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();

                    assert!(range1.len() == 2 && range2.len() == 2);
                    assert!(range1[0] <= range1[1] && range2[0] <= range2[1]);

                    ((range1[0] <= range2[0] && range2[1] <= range1[1])
                        || (range2[0] <= range1[0] && range1[1] <= range2[1]))
                        as u32
                }
                _ => panic!("invalid case"),
            }
        })
        .sum::<u32>();

    println!("{:?}", sol1);

    let sol2 = line
        .trim()
        .split('\n')
        .map(|x| {
            let words = x.split(',').collect::<Vec<&str>>();
            match words[..] {
                [str1, str2] => {
                    let range1 = str1
                        .split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();

                    let range2 = str2
                        .split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();

                    assert!(range1.len() == 2 && range2.len() == 2);
                    assert!(range1[0] <= range1[1] && range2[0] <= range2[1]);

                    !((range1[1] < range2[0]) || range2[1] < range1[0]) as u32
                }
                _ => panic!("invalid case"),
            }
        })
        .sum::<u32>();

    println!("{:?}", sol2);

    Ok(())
}
