use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const BOUND: usize = 10;

fn ex1() -> Result<(), Error> {
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day11/input.txt")?;
    let buffered = BufReader::new(input);

    let mut cavern: Vec<Vec<u8>> = vec![];
    let mut count_flash = 0;
    let mut will_flash: HashSet<(usize, usize)> = HashSet::new();
    let mut has_flashed: HashSet<(usize, usize)> = HashSet::new();

    for line in buffered.lines() {
        let values = line?;
        cavern.push(
            values
                .chars()
                .map(|value| value.to_digit(10).unwrap() as u8)
                .collect::<_>(),
        );
    }

    for _ in 0..100 {
        for i in 0..cavern.len() {
            for j in 0..BOUND {
                cavern[i][j] += 1;
                if cavern[i][j] >= 10 {
                    cavern[i][j] = 10;
                    will_flash.insert((i, j));
                }
            }
        }

        while !will_flash.is_empty() {
            for coors in will_flash.clone().iter() {
                if !has_flashed.contains(&coors) {
                    flash_update(&mut cavern, &mut will_flash, coors.0, coors.1);
                    count_flash += 1;
                    has_flashed.insert(*coors);
                }
                will_flash.remove(coors);
            }
        }

        for coors in has_flashed.drain() {
            cavern[coors.0][coors.1] = 0;
        }
    }

    println!("{}", count_flash);
    Ok(())
}

fn ex2() -> Result<(), Error> {
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day11/input.txt")?;
    let buffered = BufReader::new(input);

    let mut cavern: Vec<Vec<u8>> = vec![];
    let mut count_flash = 0;
    let mut will_flash: HashSet<(usize, usize)> = HashSet::new();
    let mut has_flashed: HashSet<(usize, usize)> = HashSet::new();

    for line in buffered.lines() {
        let values = line?;
        cavern.push(
            values
                .chars()
                .map(|value| value.to_digit(10).unwrap() as u8)
                .collect::<_>(),
        );
    }

    let mut step = 1;
    loop {
        for i in 0..cavern.len() {
            for j in 0..BOUND {
                cavern[i][j] += 1;
                if cavern[i][j] >= 10 {
                    cavern[i][j] = 10;
                    will_flash.insert((i, j));
                }
            }
        }

        while !will_flash.is_empty() {
            for coors in will_flash.clone().iter() {
                if !has_flashed.contains(&coors) {
                    flash_update(&mut cavern, &mut will_flash, coors.0, coors.1);
                    count_flash += 1;
                    has_flashed.insert(*coors);
                }
                will_flash.remove(coors);
            }
        }

        if has_flashed.len() == BOUND * cavern.len() {
            println!("{}", step);
            break;
        }
        for coors in has_flashed.drain() {
            cavern[coors.0][coors.1] = 0;
        }
        step += 1;
    }
    Ok(())
}

fn flash_update(
    cavern: &mut Vec<Vec<u8>>,
    will_flash: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) {
    for a in -1..2 {
        for b in -1..2 {
            if !(a == 0 && b == 0)
                && (x as i32 + a) >= 0
                && (x as i32 + a) < cavern.len() as i32
                && (y as i32 + b) >= 0
                && (y as i32 + b) < BOUND as i32
            {
                if cavern[(x as i32 + a) as usize][(y as i32 + b) as usize] < 10 {
                    cavern[(x as i32 + a) as usize][(y as i32 + b) as usize] += 1;
                    if cavern[(x as i32 + a) as usize][(y as i32 + b) as usize] >= 10 {
                        will_flash.insert(((x as i32 + a) as usize, (y as i32 + b) as usize));
                    }
                }
            }
        }
    }
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
