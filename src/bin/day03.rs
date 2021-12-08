use std::fs::File;
use std::io::{ BufReader, BufRead, Error};

fn ex1() -> Result<(), Error> {
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day03/input.txt")?;
    let buffered = BufReader::new(input);
    let mut line_counter = 0;
    let mut counters = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let bound = counters.len();

    for line in buffered.lines() {
        line_counter += 1;
        let values = line?.into_bytes();
        for n in 0..bound {
            match values[n] {
                49 => counters[n] += 1,
                _ => (), 
            }
        }
    }
    for n in 0..bound {
        if counters[n] >= line_counter / 2 {
            counters[n] = 1;
        } else {
            counters[n] = 0;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    let base: u32 = 2;
    for n in (0..bound).rev() {
        gamma += counters[n] * base.pow((bound - 1 - n) as u32);
        epsilon += (1 - counters[n]) * base.pow((bound - 1 - n) as u32);
    }
    println!("{}", epsilon * gamma);
    Ok(())

}

fn main() {
    ex1().unwrap();
}
