use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let line: String = fs::read_to_string(filename)?.parse()?;
    let mut map = line.trim().split('\n')
                    .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
                    .collect::<Vec<Vec<u32>>>();
    let h = map.len();
    let w = map[0].len();

    let mut sol1 = 0;
    let mut step = 0;
    loop {
        let flashes = flash(&mut map, h, w);
        if step < 100 {
            sol1 += flashes;
        }
        if flashes == h * w {
            break;
        }
        step += 1;
    }

    assert!(sol1 == 1649);
    println!("Part 1 = {}", sol1);

    assert!(step+1 == 256);
    println!("Part 2 = {}", step+1);

    Ok(())
}

fn flash(map: &mut Vec<Vec<u32>>, h: usize, w: usize) -> usize {
    
    for i in 0..h {
        for j in 0..w {
           map[i][j] += 1;
        }
    }
    
    let neighbors = [(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,-1)];
    let mut flashed = Vec::<(isize,isize)>::new();

    for i in 0..h {
        for j in 0..w {

            if map[i][j] > 9 && !flashed.contains(&(i as isize, j as isize)) {

                let mut to_flash = vec![(i as isize, j as isize)];
                while !to_flash.is_empty() {
                    if let Some((ni, nj)) = to_flash.pop() {
                        flashed.push((ni,nj));
                        for (di,dj) in neighbors {
                            if ni+di>=0 && ni+di<(h as isize) && nj+dj>=0 && nj+dj<(w as isize) {
                                map[(ni+di) as usize][(nj+dj) as usize] += 1;
                                if map[(ni+di) as usize][(nj+dj) as usize] > 9 
                                    && !flashed.contains(&(ni+di,nj+dj))
                                    && !to_flash.contains(&(ni+di,nj+dj)) {
                                    to_flash.push((ni+di,nj+dj));
                                }
                            }
                        }
                    }
                }
           }
        }
    }

    for i in 0..h {
       for j in 0..w {
           if map[i][j] > 9 {
               map[i][j] = 0;
           }
       }
    }

    flashed.len()
}
