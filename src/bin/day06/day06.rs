use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn ex1() -> Result<(), Error> {
    const PERIOD: u16 = 80;
    let input = File::open("./src/bin/day06/input.txt")?;
    let buffered = BufReader::new(input);
    // let mut m: [[u32; BOUND]; BOUND] = [[0; BOUND]; BOUND];
    let mut lines_cursor = buffered.lines();
    let first_line = lines_cursor.next().unwrap().unwrap();
    let mut fishes = first_line
        .split(',')
        .map(|value| value.parse::<i32>())
        .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
        .unwrap();

    for _ in 0..PERIOD {
        let mut new_fishes = vec![];
        for i in 0..fishes.len() {
            fishes[i] -= 1;
            if fishes[i] == -1 {
                fishes[i] = 6;
                new_fishes.push(8);
            }
        }
        fishes.append(&mut new_fishes);
    }
    println!("{}", fishes.len());

    Ok(())
}

fn ex2() -> Result<(), Error> {
    const PERIOD: u16 = 256;
    let input = File::open("./src/bin/day06/input.txt")?;
    let buffered = BufReader::new(input);
    // let mut m: [[u32; BOUND]; BOUND] = [[0; BOUND]; BOUND];
    let mut lines_cursor = buffered.lines();
    let first_line = lines_cursor.next().unwrap().unwrap();
    let fishes = first_line
        .split(',')
        .map(|value| value.parse::<i32>())
        .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
        .unwrap();

    let mut fish_bucket: [u64; 9] = [0; 9];
    for i in 0..fishes.len() {
        fish_bucket[fishes[i] as usize] += 1;
    }

    for _ in 0..PERIOD {
        let stock = fish_bucket[0];
        for i in 1..9 {
            fish_bucket[i - 1] = fish_bucket[i];
        }
        fish_bucket[6] += stock;
        fish_bucket[8] = stock;
    }
    println!("{}", fish_bucket.iter().sum::<u64>());

    Ok(())
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
