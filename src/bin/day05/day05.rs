use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const BOUND: usize = 1000;

fn update_map(m: &mut [[u32; BOUND]; BOUND], fp: Vec<u32>, sp: Vec<u32>, with_diag: bool) {
    if fp[0] == sp[0] {
        let mut start;
        let end;
        if fp[1] < sp[1] {
            start = fp[1];
            end = sp[1];
        } else {
            start = sp[1];
            end = fp[1];
        };

        while start <= end {
            m[fp[0] as usize][start as usize] += 1;
            start += 1;
        }
    } else if fp[1] == sp[1] {
        let mut start;
        let end;
        if fp[0] < sp[0] {
            start = fp[0];
            end = sp[0];
        } else {
            start = sp[0];
            end = fp[0];
        };

        while start <= end {
            m[start as usize][fp[1] as usize] += 1;
            start += 1;
        }
    } else if with_diag {
        let step_x: i32 = if fp[0] < sp[0] { 1 } else { -1 };
        let step_y: i32 = if fp[1] < sp[1] { 1 } else { -1 };
        let mut start_x: i32 = fp[0] as i32;
        let mut start_y: i32 = fp[1] as i32;

        while start_x != sp[0] as i32 {
            m[start_x as usize][start_y as usize] += 1;
            start_x += step_x;
            start_y += step_y;
        }
        m[start_x as usize][start_y as usize] += 1;
    }
}

fn print_state(m: &[[u32; BOUND]; BOUND], print_map: bool) {
    let mut counter = 0;
    for i in 0..BOUND {
        for j in 0..BOUND {
            if print_map {
                print!("{} ", m[j][i]);
            }
            if m[j][i] >= 2 {
                counter += 1
            }
        }
        if print_map {
            println!();
        };
    }
    println!("{}", counter);
}

fn ex1() -> Result<(), Error> {
    let input = File::open("./src/bin/day05/input.txt")?;
    let buffered = BufReader::new(input);
    let mut m: [[u32; BOUND]; BOUND] = [[0; BOUND]; BOUND];

    for line in buffered.lines() {
        let values = line?;
        let coordinates: Vec<String> = values
            .split(" -> ")
            .map(|value| value.to_string())
            .collect();
        let fp = coordinates[0]
            .split(',')
            .map(|value| value.parse::<u32>())
            .collect::<Result<Vec<u32>, <u32 as std::str::FromStr>::Err>>()
            .unwrap();
        let sp = coordinates[1]
            .split(',')
            .map(|value| value.parse::<u32>())
            .collect::<Result<Vec<u32>, <u32 as std::str::FromStr>::Err>>()
            .unwrap();
        update_map(&mut m, fp, sp, false);
    }

    print_state(&m, false);
    Ok(())
}

fn ex2() -> Result<(), Error> {
    let input = File::open("./src/bin/day05/input.txt")?;
    let buffered = BufReader::new(input);
    let mut m: [[u32; BOUND]; BOUND] = [[0; BOUND]; BOUND];

    for line in buffered.lines() {
        let values = line?;
        let coordinates: Vec<String> = values
            .split(" -> ")
            .map(|value| value.to_string())
            .collect();
        let fp = coordinates[0]
            .split(',')
            .map(|value| value.parse::<u32>())
            .collect::<Result<Vec<u32>, <u32 as std::str::FromStr>::Err>>()
            .unwrap();
        let sp = coordinates[1]
            .split(',')
            .map(|value| value.parse::<u32>())
            .collect::<Result<Vec<u32>, <u32 as std::str::FromStr>::Err>>()
            .unwrap();
        update_map(&mut m, fp, sp, true);
    }

    print_state(&m, false);
    Ok(())
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
