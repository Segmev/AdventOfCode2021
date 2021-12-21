use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn ex1() -> Result<(), Error> {
    let input = File::open("./src/bin/day13/input.txt")?;
    let buffered = BufReader::new(input);
    let mut lines_cursor = buffered.lines();
    let mut sheet = HashSet::new();
    loop {
        match lines_cursor.next() {
            Some(unwl) => {
                let l = unwl.unwrap();
                if l.len() < 3 {
                    break;
                }
                let coors = l
                    .split(',')
                    .map(|value| value.parse::<i32>())
                    .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
                    .unwrap();
                sheet.insert((coors[0], coors[1]));
            }
            _ => break,
        }
    }
    let mut instructions = vec![];
    loop {
        match lines_cursor.next() {
            Some(unwl) => {
                instructions.push(
                    unwl.unwrap()
                        .split(" ")
                        .map(|value| value.to_string())
                        .collect::<Vec<String>>()[2]
                        .clone(),
                );
            }
            _ => break,
        }
    }
    fold_once(&mut sheet, instructions[0].clone());
    println!("{}", sheet.len());
    Ok(())
}

fn ex2() -> Result<(), Error> {
    let input = File::open("./src/bin/day13/input.txt")?;
    let buffered = BufReader::new(input);
    let mut lines_cursor = buffered.lines();
    let mut sheet = HashSet::new();
    loop {
        match lines_cursor.next() {
            Some(unwl) => {
                let l = unwl.unwrap();
                if l.len() < 3 {
                    break;
                }
                let coors = l
                    .split(',')
                    .map(|value| value.parse::<i32>())
                    .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
                    .unwrap();
                sheet.insert((coors[0], coors[1]));
            }
            _ => break,
        }
    }
    let mut instructions = vec![];
    loop {
        match lines_cursor.next() {
            Some(unwl) => {
                instructions.push(
                    unwl.unwrap()
                        .split(" ")
                        .map(|value| value.to_string())
                        .collect::<Vec<String>>()[2]
                        .clone(),
                );
            }
            _ => break,
        }
    }
    for instruction in instructions {
        fold_once(&mut sheet, instruction.clone());
    }
    display_sheet(&sheet);
    Ok(())
}

fn fold_once(sheet: &mut HashSet<(i32, i32)>, instruction: String) {
    let splitted_instruction = instruction
        .split("=")
        .map(|v| v.to_string())
        .collect::<Vec<_>>();
    let axis = &splitted_instruction[0][..];
    let coor_fold = splitted_instruction[1].parse::<i32>().unwrap();
    let mut drained_coors = HashSet::new();
    match axis {
        "x" => {
            for coors in sheet.drain() {
                if coors.0 < coor_fold {
                    drained_coors.insert(coors);
                } else if coors.0 - coor_fold >= 0 {
                    drained_coors.insert((coor_fold - (coors.0 - coor_fold), coors.1));
                }
            }
        }
        "y" => {
            for coors in sheet.drain() {
                if coors.1 < coor_fold {
                    drained_coors.insert(coors);
                } else if coors.1 - coor_fold >= 0 {
                    drained_coors.insert((coors.0, coor_fold - (coors.1 - coor_fold)));
                }
            }
        }
        _ => (),
    }
    sheet.extend(&drained_coors);
}

fn display_sheet(sheet: &HashSet<(i32, i32)>) {
    let bound_x = 39;
    let bound_y = 6;
    for j in 0..bound_y {
        for i in 0..bound_x {
            if sheet.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
