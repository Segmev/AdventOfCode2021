use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn ex1() -> Result<(), Error> {
    let input = File::open("./src/bin/day14/input.txt")?;
    let buffered = BufReader::new(input);
    let mut lines_cursor = buffered.lines();
    let mut rules = HashMap::new();
    let mut elements_in_polymere: HashMap<char, u64> = HashMap::new();
    let mut poly_template = lines_cursor.next().unwrap().unwrap();
    lines_cursor.next();
    loop {
        match lines_cursor.next() {
            Some(unwl) => {
                let l = unwl.unwrap();
                if l.len() < 3 {
                    break;
                }
                let coors = l
                    .split(" -> ")
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>();
                let mut entry_chars = coors[0].chars();
                rules.insert(
                    (entry_chars.next().unwrap(), entry_chars.next().unwrap()),
                    coors[1].chars().next().unwrap(),
                );
            }
            _ => break,
        }
    }

    for _ in 0..10 {
        let chars = poly_template.chars().collect::<Vec<char>>();
        let mut next_polymere = String::new();
        next_polymere.push(chars[0]);
        for i in 1..chars.len() {
            match rules.get(&(chars[i - 1], chars[i])) {
                Some(letter) => next_polymere.push(*letter),
                _ => (),
            }
            next_polymere.push(chars[i]);
        }
        poly_template = next_polymere;
    }

    for letter in poly_template.chars() {
        *elements_in_polymere.entry(letter).or_insert(0) += 1;
    }
    let mut least_elements = 9900000;
    let mut most_elements = 0;
    for entry in elements_in_polymere.iter() {
        if *entry.1 > most_elements {
            most_elements = *entry.1;
        }
        if *entry.1 < least_elements {
            least_elements = *entry.1;
        }
    }
    println!("{}", most_elements - least_elements);
    Ok(())
}

fn ex2() -> Result<(), Error> {
    let input = File::open("./src/bin/day14/input.txt")?;
    let buffered = BufReader::new(input);
    let mut lines_cursor = buffered.lines();
    let mut rules = HashMap::new();
    let mut poly_paires: HashMap<(char, char), u64> = HashMap::new();
    let poly_template = lines_cursor.next().unwrap().unwrap();
    lines_cursor.next();
    loop {
        match lines_cursor.next() {
            Some(unwl) => {
                let l = unwl.unwrap();
                if l.len() < 3 {
                    break;
                }
                let coors = l
                    .split(" -> ")
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>();
                let mut entry_chars = coors[0].chars();
                rules.insert(
                    (entry_chars.next().unwrap(), entry_chars.next().unwrap()),
                    coors[1].chars().next().unwrap(),
                );
            }
            _ => break,
        }
    }

    let chars = poly_template.chars().collect::<Vec<char>>();
    for i in 1..chars.len() {
        *poly_paires.entry((chars[i - 1], chars[i])).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut new_poly_pairs = poly_paires.clone();
        for (key, val) in poly_paires.iter() {
            match rules.get(&(key)) {
                Some(letter) => {
                    match new_poly_pairs.get_mut(key) {
                        Some(x) => *x = if *x >= *val { *x - *val } else { 0 },
                        _ => (),
                    };
                    *new_poly_pairs.entry((key.0, *letter)).or_insert(0) += *val;
                    *new_poly_pairs.entry((*letter, key.1)).or_insert(0) += *val;
                }
                _ => {}
            }
        }
        poly_paires = new_poly_pairs;
    }
    let mut elements_in_polymere: HashMap<char, u64> = HashMap::new();
    let mut most_elements = 0;
    let mut least_elements = u64::MAX;

    let mut first_elem_loop = true;
    for (key, val) in poly_paires.iter() {
        if first_elem_loop {
            *elements_in_polymere.entry(key.0).or_insert(0) += *val;
            first_elem_loop = false;
        }
        *elements_in_polymere.entry(key.1).or_insert(0) += *val;
    }

    for entry in elements_in_polymere.iter() {
        if *entry.1 > most_elements {
            most_elements = *entry.1;
        }
        if *entry.1 < least_elements {
            least_elements = *entry.1;
        }
    }

    println!("{}", most_elements - least_elements);
    Ok(())
}
fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
