use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn ex1() -> Result<(), Error> {
    let input = File::open("./src/bin/day07/input.txt")?;
    let buffered = BufReader::new(input);
    // let mut m: [[u32; BOUND]; BOUND] = [[0; BOUND]; BOUND];
    let mut lines_cursor = buffered.lines();
    let first_line = lines_cursor.next().unwrap().unwrap();
    let levels = first_line
        .split(',')
        .map(|value| value.parse::<i32>())
        .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
        .unwrap();

    let mut final_fuel = i32::MAX;
    for i in 0..levels.len() {
        let mut fuel = 0;
        for j in 0..levels.len() {
            fuel += (levels[i] - levels[j]).abs();
        }
        if fuel < final_fuel {
            final_fuel = fuel;
        }
    }
    println!("{}", final_fuel);
    Ok(())
}

fn average(numbers: &Vec<i32>) -> i32 {
    (numbers.iter().sum::<i32>() as f32 / numbers.len() as f32) as i32
}

fn ex2() -> Result<(), Error> {
    let input = File::open("./src/bin/day07/input.txt")?;
    let buffered = BufReader::new(input);
    // let mut m: [[u32; BOUND]; BOUND] = [[0; BOUND]; BOUND];
    let mut lines_cursor = buffered.lines();
    let first_line = lines_cursor.next().unwrap().unwrap();
    let levels = first_line
        .split(',')
        .map(|value| value.parse::<i32>())
        .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
        .unwrap();

    let mut steps: [u32; 1500] = [0; 1500];
    for i in 1..steps.len() {
        steps[i] = steps[i - 1] + (i as u32);
    }
    let av = average(&levels);

    let mut final_fuel = u32::MAX;
    for i in (av - 10)..(av + 10) {
        let mut fuel = 0;
        for j in 0..levels.len() {
            fuel += steps[(i - levels[j]).abs() as usize];
        }
        if fuel < final_fuel {
            final_fuel = fuel;
        }
    }
    println!("{}", final_fuel);
    Ok(())
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
