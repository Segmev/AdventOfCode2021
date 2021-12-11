use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const BOUND: usize = 12;
const BASE: u32 = 2;

fn ex1() -> Result<(), Error> {
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day03/input.txt")?;
    let buffered = BufReader::new(input);
    let mut line_counter = 0;

    let mut counters: [u32; BOUND] = [0; BOUND];

    for line in buffered.lines() {
        line_counter += 1;
        let values = line?.into_bytes();
        for n in 0..BOUND {
            match values[n] {
                49 => counters[n] += 1,
                _ => (),
            }
        }
    }
    for n in 0..BOUND {
        if counters[n] >= line_counter / 2 {
            counters[n] = 1;
        } else {
            counters[n] = 0;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for n in (0..BOUND).rev() {
        gamma += counters[n] * BASE.pow((BOUND - 1 - n) as u32);
        epsilon += (1 - counters[n]) * BASE.pow((BOUND - 1 - n) as u32);
    }
    println!("{}", epsilon * gamma);
    Ok(())
}

fn ex2() -> Result<(), Error> {
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day03/input.txt")?;
    let buffered = BufReader::new(input);

    let mut bnums: Vec<[u32; BOUND]> = vec![];
    for line in buffered.lines() {
        let mut counters: [u32; BOUND] = [0; BOUND];
        let values = line?.into_bytes();
        for n in 0..BOUND {
            match values[n] {
                49 => counters[n] = 1,
                _ => (),
            }
        }
        bnums.push(counters);
    }
    let mut oxygen_vec = bnums.clone();
    for n in 0..BOUND {
        if oxygen_vec.len() == 1 {
            break;
        }
        let mut zero_indexes: Vec<usize> = vec![];
        let mut one_indexes: Vec<usize> = vec![];
        for i in 0..oxygen_vec.len() {
            match oxygen_vec[i][n] {
                0 => zero_indexes.push(i),
                1 => one_indexes.push(i),
                _ => (),
            }
        }
        if one_indexes.len() >= zero_indexes.len() {
            for i in (0..zero_indexes.len()).rev() {
                oxygen_vec.remove(zero_indexes[i]);
            }
        } else {
            for i in (0..one_indexes.len()).rev() {
                oxygen_vec.remove(one_indexes[i]);
            }
        }
    }

    let mut co2_vec = bnums.clone();
    for n in 0..BOUND {
        if co2_vec.len() == 1 {
            break;
        }
        let mut zero_indexes: Vec<usize> = vec![];
        let mut one_indexes: Vec<usize> = vec![];
        for i in 0..co2_vec.len() {
            match co2_vec[i][n] {
                0 => zero_indexes.push(i),
                1 => one_indexes.push(i),
                _ => (),
            }
        }
        if one_indexes.len() >= zero_indexes.len() {
            for i in (0..one_indexes.len()).rev() {
                co2_vec.remove(one_indexes[i]);
            }
        } else {
            for i in (0..zero_indexes.len()).rev() {
                co2_vec.remove(zero_indexes[i]);
            }
        }
    }

    let mut oxy = 0;
    let mut co2 = 0;
    for n in (0..BOUND).rev() {
        oxy += oxygen_vec[0][n] * BASE.pow((BOUND - 1 - n) as u32);
        co2 += co2_vec[0][n] * BASE.pow((BOUND - 1 - n) as u32);
    }

    println!("{}", oxy * co2);
    Ok(())
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
