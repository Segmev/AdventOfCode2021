use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn ex1() -> Result<(), Error> {
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day08/input.txt")?;
    let buffered = BufReader::new(input);

    let mut count = 0;
    for line in buffered.lines() {
        let values = line?;
        let output: Vec<String> = values.split(" | ").map(|value| value.to_string()).collect();
        let signal_patterns = output[1]
            .split(' ')
            .map(|value| value.to_string())
            .collect::<Vec<String>>();

        for pattern in signal_patterns {
            if pattern.len() <= 4 || pattern.len() == 7 {
                count += 1;
            }
        }
    }
    println!("{}", count);

    Ok(())
}

fn match_sign(sign_map: &HashMap<usize, Vec<char>>, signal: String) -> i32 {
    return match signal.len() {
        2 => 1,
        4 => 4,
        3 => 7,
        7 => 8,
        5 => {
            let mut countfor3 = 0;
            let mut countfor2 = 0;
            for c in signal.chars() {
                if sign_map[&(2)].contains(&c) {
                    countfor3 += 1;
                }
                if sign_map[&(4)].contains(&c) {
                    countfor2 += 1;
                }
            }
            if countfor3 == 2 {
                return 3;
            }
            if countfor2 == 2 {
                return 2;
            }
            return 5;
        }
        6 => {
            let mut countfor6 = 0;
            let mut countfor9 = 0;
            for c in signal.chars() {
                if sign_map[&(2)].contains(&c) {
                    countfor6 += 1;
                }
                if sign_map[&(4)].contains(&c) {
                    countfor9 += 1;
                }
            }
            if countfor6 == 1 {
                return 6;
            }
            if countfor9 == 4 {
                return 9;
            }

            return 0;
        }
        _ => -999999,
    };
}

fn ex2() -> Result<(), Error> {
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day08/input.txt")?;
    let buffered = BufReader::new(input);

    let mut sum = 0;
    for line in buffered.lines() {
        let values = line?;
        let display_l: Vec<String> = values.split(" | ").map(|value| value.to_string()).collect();

        let mut sign_map = HashMap::with_capacity(3);

        for pattern in display_l[0].split(' ') {
            if 2 <= pattern.len() && pattern.len() <= 4 {
                sign_map.insert(pattern.len(), pattern.chars().collect::<Vec<char>>());
            }
        }
        let signal_outputs = display_l[1]
            .split(' ')
            .map(|value| value.to_string())
            .collect::<Vec<String>>();

        let mut v = 0;
        for output in signal_outputs {
            v = v * 10;
            v += match_sign(&sign_map, output);
        }
        sum += v;
    }
    println!("{}", sum);
    Ok(())
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
