use std::fs;
use std::env;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let lines: Vec<String> = fs::read_to_string(filename)?.split("\n").map(|s| s.to_string()).collect();

    let mut sol1 = 0;
    let mut sol2 = 0;
    for line in lines {
        if let Some((part1, part2)) = line.split_once(" | ") {
            let mut dig1: Vec<String> = part1.split(" ").map(|x| sort_str(x)).collect();
            let mut dig2: Vec<String> = part2.split(" ").map(|x| sort_str(x)).collect();
            sol1 += dig2
                .iter()
                .filter(|x| x.len()==2 || x.len()==3 || x.len()==4 || x.len()==7)
                .count();
            dig1.append(&mut dig2);
            sol2 += convert(&dig1);
        }
    }

    assert!(sol1 == 514);
    println!("Part 1 = {}", sol1);
    assert!(sol2 == 1012272);
    println!("Part 2 = {}", sol2);

    Ok(())
}

fn sort_str(str_: &str) -> String {
    let wordy = String::from(str_);
    let mut chars: Vec<char> = wordy.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(chars)
}

fn convert(strings: &Vec<String>) -> u32 {

    let mut map: HashMap<String, u8> = HashMap::new();
    let mut map_i: HashMap<u8, String> = HashMap::new();
    let mut cf = String::new();
    let mut bd = String::new();
    for i in [1, 7, 4, 8, 6, 0, 9, 5, 3, 2] {
        for s in strings {
            if i==1 && s.len()==2 {
                cf = s.to_string();
                map.entry(s.to_string()).or_insert(i);
                break;
            }
            else if i==7 && s.len()==3 {
                map.entry(s.to_string()).or_insert(i);
                map_i.entry(i).or_insert(s.to_string());
                break;
            } else if i==4 && s.len()==4 {
                bd = diff(&s, &cf);
                map.entry(s.to_string()).or_insert(i);
                map_i.entry(i).or_insert(s.to_string());
                break;
            } else if i==8 && s.len()==7 {
                map.entry(s.to_string()).or_insert(i);
                map_i.entry(i).or_insert(s.to_string());
                break;
            } else if i==6 && s.len()==6 && (s.find(cf.chars().nth(0).unwrap()) == None || s.find(cf.chars().nth(1).unwrap()) == None) {
                map.entry(s.to_string()).or_insert(i);
                map_i.entry(i).or_insert(s.to_string());
                break;
            } else if i==0 && s.len()==6 && (s.find(bd.chars().nth(0).unwrap()) == None || s.find(bd.chars().nth(1).unwrap()) == None) {
                map.entry(s.to_string()).or_insert(i);
                map_i.entry(i).or_insert(s.to_string());
                break;
            } else if i==9 && s.len()==6 && *s != map_i[&6] && *s != map_i[&0] {
                map.entry(s.to_string()).or_insert(i);
                map_i.entry(i).or_insert(s.to_string());
                break;
            } else if i==5 && s.len()==5 && (s.find(bd.chars().nth(0).unwrap()) != None && s.find(bd.chars().nth(1).unwrap()) != None) {
                map.entry(s.to_string()).or_insert(i);
                map_i.entry(i).or_insert(s.to_string());
                break;
            } else if i==3 && s.len()==5 && (s.find(cf.chars().nth(0).unwrap()) != None && s.find(cf.chars().nth(1).unwrap()) != None) {
                map.entry(s.to_string()).or_insert(i);
                map_i.entry(i).or_insert(s.to_string());
                break;
            } else if i==2 && s.len()==5 && *s != map_i[&5] && *s != map_i[&3] {
                map.entry(s.to_string()).or_insert(i);
                map_i.entry(i).or_insert(s.to_string());
                break;
            }
        }
    }

    (1..=4).rev().map(|i| (map[&strings[strings.len()-i]] as u32) * 10_u32.pow((i-1) as u32)).sum()
}

fn diff(a_: &str, b_: &str) -> String {
    let a;
    let b;
    if a_.len() < b_.len() {
        a = b_.to_string();
        b = a_.to_string();
    } else {
        a = a_.to_string();
        b = b_.to_string();
    }
    let mut diff_s = String::new();
    for i in a.chars() {
        if let None = b.find(i) {
            diff_s.push(i);
        }
    }
    diff_s
}