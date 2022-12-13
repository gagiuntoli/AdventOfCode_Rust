use std::cmp::min;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let file: String = fs::read_to_string(filename)?.parse()?;

    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;
    let mut possible_starts = Vec::<(usize, usize)>::new();

    let map = file
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' || c == 'a' {
                        possible_starts.push((i, j));
                    }
                    if c == 'S' {
                        start = Some((i, j));
                        'a'
                    } else if c == 'E' {
                        end = Some((i, j));
                        'z'
                    } else {
                        c
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let start = start.unwrap();
    let end = end.unwrap();

    println!("{}", dijkstra(&map, start, end));
    println!(
        "{}",
        possible_starts
            .iter()
            .map(|start| dijkstra(&map, *start, end))
            .min()
            .unwrap()
    );

    Ok(())
}

fn get_neighbors(current: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::<(usize, usize)>::new();
    if current.0 > 0 {
        neighbors.push((current.0 - 1, current.1));
    }
    if current.0 < map.len() - 1 {
        neighbors.push((current.0 + 1, current.1));
    }
    if current.1 > 0 {
        neighbors.push((current.0, current.1 - 1));
    }
    if current.1 < map[0].len() - 1 {
        neighbors.push((current.0, current.1 + 1));
    }
    neighbors
}

fn dijkstra(map: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut visited = HashSet::<(usize, usize)>::new();
    let inf = 1000000000;
    let mut dist = vec![vec![inf; map[0].len()]; map.len()];

    dist[start.0][start.1] = 0;

    let mut current = start;
    visited.insert(current);

    while current != end {
        let neighbors = get_neighbors(current, &map);

        neighbors.iter().for_each(|node| {
            let height_diff = map[node.0][node.1] as i32 - map[current.0][current.1] as i32;

            if !visited.contains(node) {
                if height_diff <= 1 {
                    dist[node.0][node.1] =
                        min(dist[node.0][node.1], dist[current.0][current.1] + 1);
                }
            }
        });

        let mut next_current: Option<(usize, usize)> = None;
        let mut min_dist = inf;
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if dist[i][j] < min_dist && !visited.contains(&(i, j)) {
                    next_current = Some((i, j));
                    min_dist = dist[i][j];
                }
            }
        }
        visited.insert(current);
        if next_current == None {
            return 100000000;
        } else {
            current = next_current.unwrap();
        }
    }
    dist[end.0][end.1]
}
