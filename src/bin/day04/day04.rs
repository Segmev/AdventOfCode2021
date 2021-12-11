use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn update_board(board: &mut Vec<Vec<(i32, bool)>>, number: i32) {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j].0 == number {
                board[i][j].1 = true;
            }
        }
    }
}

fn is_bingo(board: &Vec<Vec<(i32, bool)>>) -> bool {
    for i in 0..5 {
        let mut count_marked = 0;
        for j in 0..5 {
            if board[i][j].1 {
                count_marked += 1;
            }
        }
        if count_marked == 5 {
            return true;
        }
    }
    for i in 0..5 {
        let mut count_marked = 0;
        for j in 0..5 {
            if board[j][i].1 {
                count_marked += 1;
            }
        }
        if count_marked == 5 {
            return true;
        }
    }

    false
}

fn calculate_unmarked_numbers(board: &Vec<Vec<(i32, bool)>>) -> i32 {
    let mut acc = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !board[i][j].1 {
                acc += board[i][j].0;
            }
        }
    }
    acc
}

fn ex1() -> Result<(), Error> {
    let re = Regex::new(r"\s+").unwrap();
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day04/input.txt")?;
    let buffered = BufReader::new(input);

    let mut lines_cursor = buffered.lines();
    let first_line = lines_cursor.next().unwrap().unwrap();
    let values = first_line
        .split(',')
        .map(|value| value.parse::<i32>())
        .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
        .unwrap();
    println!("{:?}", values);

    let mut boards: Vec<Vec<Vec<(i32, bool)>>> = vec![];

    loop {
        match lines_cursor.next() {
            Some(_) => {
                let mut board: Vec<Vec<(i32, bool)>> = vec![];
                for _ in 0..5 {
                    let number_line = re
                        .replace_all(lines_cursor.next().unwrap().unwrap().trim(), " ")
                        .into_owned();
                    let r: Vec<i32> = number_line
                        .split(" ")
                        .map(|value| value.parse::<i32>())
                        .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
                        .unwrap();
                    board.push(r.iter().cloned().map(|value| (value, false)).collect());
                }
                boards.push(board);
            }
            None => break,
        }
    }
    for number in values {
        for i in 0..boards.len() {
            update_board(&mut boards[i], number);
            if is_bingo(&boards[i]) {
                println!(
                    "Bingo ! {:?}",
                    number * calculate_unmarked_numbers(&boards[i])
                );
                return Ok(());
            }
        }
    }
    Ok(())
}

fn ex2() -> Result<(), Error> {
    let re = Regex::new(r"\s+").unwrap();
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day04/input.txt")?;
    let buffered = BufReader::new(input);

    let mut lines_cursor = buffered.lines();
    let first_line = lines_cursor.next().unwrap().unwrap();
    let values = first_line
        .split(',')
        .map(|value| value.parse::<i32>())
        .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
        .unwrap();
    println!("{:?}", values);

    let mut boards: Vec<Vec<Vec<(i32, bool)>>> = vec![];

    loop {
        match lines_cursor.next() {
            Some(_) => {
                let mut board: Vec<Vec<(i32, bool)>> = vec![];
                for _ in 0..5 {
                    let number_line = re
                        .replace_all(lines_cursor.next().unwrap().unwrap().trim(), " ")
                        .into_owned();
                    let r: Vec<i32> = number_line
                        .split(" ")
                        .map(|value| value.parse::<i32>())
                        .collect::<Result<Vec<i32>, <i32 as std::str::FromStr>::Err>>()
                        .unwrap();
                    board.push(r.iter().cloned().map(|value| (value, false)).collect());
                }
                boards.push(board);
            }
            None => break,
        }
    }
    let mut winning_boards: Vec<usize> = vec![];
    for number in values {
        for i in 0..boards.len() {
            update_board(&mut boards[i], number);
            if !(winning_boards.contains(&i)) && is_bingo(&boards[i]) {
                println!(
                    "Bingo ! {:?}",
                    number * calculate_unmarked_numbers(&boards[i])
                );
                winning_boards.push(i);
            }
        }
    }
    Ok(())
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
