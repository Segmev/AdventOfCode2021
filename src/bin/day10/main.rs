use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn eval_line(dic: &HashMap<char, u32>, line: &str) -> u32 {
    let chars = line.chars().collect::<Vec<char>>();
    let mut stack: Vec<char> = vec![];

    let starters = ['<', '(', '{', '['];

    for c in chars {
        if starters.contains(&c) {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => (),
            }
        } else {
            let poped = stack.pop().unwrap();
            if poped != c {
                return dic[&c];
            }
        }
    }

    0
}

fn ex1() -> Result<(), Error> {
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day10/input.txt")?;
    let buffered = BufReader::new(input);
    let mut dic: HashMap<char, u32> = HashMap::new();
    dic.insert(')', 3);
    dic.insert(']', 57);
    dic.insert('}', 1197);
    dic.insert('>', 25137);

    let mut score = 0;
    for line in buffered.lines() {
        let values = line?;
        score += eval_line(&dic, &values);
    }
    println!("{}", score);

    Ok(())
}

fn ex2() -> Result<(), Error> {
    let input = File::open("/home/skarraz/Projects/AdventOfCode2021/src/bin/day10/input.txt")?;
    let buffered = BufReader::new(input);
    let mut dic: HashMap<char, u32> = HashMap::new();
    dic.insert(')', 3);
    dic.insert(']', 57);
    dic.insert('}', 1197);
    dic.insert('>', 25137);

    let mut scores = vec![];
    for line in buffered.lines() {
        let values = line?;
        if eval_line(&dic, &values) == 0 {
            scores.push(eval_incomplet(&values));
        }
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);

    Ok(())
}

fn eval_incomplet(line: &str) -> u64 {
    let chars = line.chars().collect::<Vec<char>>();
    let mut stack: Vec<char> = vec![];

    let starters = ['<', '(', '{', '['];

    for c in chars {
        if starters.contains(&c) {
            stack.push(c);
        } else {
            stack.pop();
        }
    }
    let mut line_score = 0;
    for c in stack.iter().rev() {
        line_score *= 5;
        line_score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        }
    }

    line_score
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
