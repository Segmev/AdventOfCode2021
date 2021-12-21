use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn ex1() -> Result<(), Error> {
    let input = File::open("./src/bin/day02/input.txt")?;
    let buffered = BufReader::new(input);
    let mut h_pos = 0;
    let mut depth = 0;
    for line in buffered.lines() {
        let unwrapped_line = line.unwrap();
        let splitted_line = unwrapped_line.split(" ");
        let entries: Vec<&str> = splitted_line.collect();

        let nb = entries[1].parse::<i32>().unwrap_or(0);
        match entries[0] {
            "forward" => h_pos += nb,
            "down" => depth += nb,
            "up" => depth -= nb,
            _ => println!("what?"),
        }
    }
    println!("{}", h_pos * depth);
    Ok(())
}

fn ex2() -> Result<(), Error> {
    let input = File::open("./src/bin/day02/input.txt")?;
    let buffered = BufReader::new(input);
    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in buffered.lines() {
        let unwrapped_line = line.unwrap();
        let splitted_line = unwrapped_line.split(" ");
        let entries: Vec<&str> = splitted_line.collect();

        let nb = entries[1].parse::<i32>().unwrap_or(0);
        match entries[0] {
            "forward" => {
                h_pos += nb;
                depth += aim * nb;
            }
            "down" => aim += nb,
            "up" => aim -= nb,
            _ => println!("what?"),
        }
    }
    println!("{}", h_pos * depth);
    Ok(())
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
