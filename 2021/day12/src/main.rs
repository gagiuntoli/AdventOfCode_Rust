use std::fs;
use std::env;
use std::error::Error;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let input = line.trim().split('\n')
                    .map(|x| {
                        match x.split_once("-") {
                            Some((a,b)) => (a.to_string(),b.to_string()),
                            None => unreachable!()
                        }
                    })
                    .collect::<Vec<(String, String)>>();

    let mut connections = HashMap::<String, Vec<String>>::new();
    let mut visited = HashMap::<String,u32>::new();
    for (key, val) in input {
        visited.entry(key.clone()).or_insert(0);
        visited.entry(val.clone()).or_insert(0);
        match connections.entry(key.clone()) {
            Entry::Vacant(e) => {e.insert(vec![val.clone()]);},
            Entry::Occupied(mut e) => {e.get_mut().push(val.clone());},
        }
        match connections.entry(val) {
            Entry::Vacant(e) => {e.insert(vec![key]);},
            Entry::Occupied(mut e) => {e.get_mut().push(key);},
        }
    }

    let sol1 = number_of_paths(&connections, "start", &mut visited.clone(), "part1");
    assert!(sol1 == 3679);
    println!("Part 1 = {}", sol1);

    let sol2 = number_of_paths(&connections, "start", &mut visited.clone(), "part2");
    assert!(sol2 == 107395);
    println!("Part 2 = {}", sol2);

    Ok(())
}

// TODO: Bad performance 
fn number_of_paths(connections: &HashMap<String, Vec<String>>, node: &str, visited: &mut HashMap<String, u32>, part: &str) -> u32 {

    if node == "end" {
        return 1;
    } else if part == "part1" && visited[node] > 0 {
        return 0;
    } else if part == "part2" && visited.values().filter(|&x| *x == 2).count() > 0 && visited[node] >= 1 {
            return 0;
    } else if node.chars().all(|x| x.is_lowercase()) {
        *visited.get_mut(node).unwrap() += 1;
    }

    let mut count = 0;
    for n in &connections[node] {
        if *n != "start" {
			    count += number_of_paths(connections, n, &mut visited.clone(), part);
        }
    }
    count
}