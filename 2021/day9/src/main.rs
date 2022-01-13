use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let map = line.trim().split('\n')
                    .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
                    .collect::<Vec<Vec<u32>>>();

    let neighbors = [(0,-1), (0,1), (1,0), (-1,0)];
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let mut sol1 = 0;
    let mut vec2 = vec![];
    for i in 0..height {
        for j in 0..width {
            if neighbors.iter().all(|&(dj, di)| {
                if j+dj >= width || j+dj < 0 || i+di >= height || i+di < 0 {
                    return true;
                } else if map[(i+di) as usize][(j+dj) as usize] > map[i as usize][j as usize] {
                    return true;
                }
                return false;
            }) {
                sol1 += map[i as usize][j as usize] + 1;
                let mut visited = Vec::<(isize, isize)>::new();
                vec2.push(basin(&map, &mut visited, i, j));
            }
        }
    }
    vec2.sort_unstable();

    assert!(sol1 == 560);
    println!("Part 1 = {}", sol1);

    let sol2 = vec2.iter().rev().take(3).product::<u32>();
    assert!(sol2 == 959136);
    println!("Part 2 = {}", sol2);

    Ok(())
}

fn basin(map: &Vec<Vec<u32>>, visited: &mut Vec<(isize, isize)>, i: isize, j: isize) -> u32 {
    
    let width = map[0].len() as isize;
    let height = map.len() as isize;
    let neighbors = [(0,-1), (0,1), (1,0), (-1,0)];
    let mut sum = 0;
    for (dj, di) in neighbors {
        if j+dj < width && j+dj >= 0 && i+di < height && i+di >= 0 
            && !visited.contains(&(i+di,j+dj)) 
            && map[(i+di) as usize][(j+dj) as usize] < 9 {
            visited.push((i+di,j+dj));
            sum += 1 + basin(map, visited, i+di, j+dj);
        }
    }
    sum
}