use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn ex1() -> Result<(), Error> {
    let input = File::open("./src/bin/day01/input.txt")?;
    let buffered = BufReader::new(input);
    let mut count = 0;
    let mut prev_nb = 10000000;

    for line in buffered.lines() {
        let nb = line?.parse::<i32>().unwrap_or(0);
        if nb > prev_nb {
            count += 1;
        }
        prev_nb = nb;
    }
    println!("{}", count);
    Ok(())
}

fn ex2() -> Result<(), Error> {
    let input = File::open("./src/bin/day01/input.txt")?;
    let buffered = BufReader::new(input);
    let mut prev_nbs: std::vec::Vec<i32> = vec![];
    let mut count = 0;

    for line in buffered.lines() {
        let nb = line?.parse::<i32>().unwrap_or(0);
        if prev_nbs.len() >= 3 {
            let prev_sum: i32 = prev_nbs[0] + prev_nbs[1] + prev_nbs[2];
            let actual_sum = nb + prev_nbs[1] + prev_nbs[2];
            if prev_sum < actual_sum {
                count += 1;
            }
            prev_nbs.drain(0..1);
        }
        prev_nbs.push(nb);
    }
    println!("{}", count);
    Ok(())
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
