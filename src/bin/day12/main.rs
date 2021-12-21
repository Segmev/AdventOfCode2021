use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct Node {
    id: String,
    is_big: bool,
    linked_nodes: Vec<String>,
}

fn ex1() -> Result<(), Error> {
    let mut network = HashMap::new();
    let input = File::open("./src/bin/day12/input.txt")?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let values = line?;
        let names: Vec<String> = values.split("-").map(|value| value.to_string()).collect();
        if !network.contains_key(&names[0]) {
            network.insert(
                names[0].clone(),
                Node {
                    id: names[0].clone(),
                    is_big: names[0].chars().next().unwrap().is_uppercase(),
                    linked_nodes: vec![names[1].clone()],
                },
            );
        } else {
            network
                .get_mut(&names[0])
                .unwrap()
                .linked_nodes
                .push(names[1].clone());
        }
        if !network.contains_key(&names[1]) {
            network.insert(
                names[1].clone(),
                Node {
                    id: names[1].clone(),
                    is_big: names[1].chars().next().unwrap().is_uppercase(),
                    linked_nodes: vec![names[0].clone()],
                },
            );
        } else {
            network
                .get_mut(&names[1])
                .unwrap()
                .linked_nodes
                .push(names[0].clone());
        }
    }
    let res = crawl_no_time(&network, network.get("start").unwrap(), &mut HashSet::new());
    println!("{}", res);

    Ok(())
}

fn crawl_no_time(
    network: &HashMap<String, Node>,
    actual_cave: &Node,
    visited_small_caves: &mut HashSet<String>,
) -> i32 {
    if actual_cave.id == "end" {
        return 1;
    }
    if !actual_cave.is_big {
        visited_small_caves.insert(actual_cave.id.clone());
    }
    let mut count = 0;
    for cave in actual_cave.linked_nodes.iter() {
        if !visited_small_caves.contains(cave) {
            count += crawl_no_time(network, &network[cave], &mut visited_small_caves.clone());
        }
    }
    count
}

fn ex2() -> Result<(), Error> {
    let mut network = HashMap::new();
    let input = File::open("./src/bin/day12/input.txt")?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let values = line?;
        let names: Vec<String> = values.split("-").map(|value| value.to_string()).collect();
        if !network.contains_key(&names[0]) {
            network.insert(
                names[0].clone(),
                Node {
                    id: names[0].clone(),
                    is_big: names[0].chars().next().unwrap().is_uppercase(),

                    linked_nodes: vec![names[1].clone()],
                },
            );
        } else {
            network
                .get_mut(&names[0])
                .unwrap()
                .linked_nodes
                .push(names[1].clone());
        }
        if !network.contains_key(&names[1]) {
            network.insert(
                names[1].clone(),
                Node {
                    id: names[1].clone(),
                    is_big: names[1].chars().next().unwrap().is_uppercase(),
                    linked_nodes: vec![names[0].clone()],
                },
            );
        } else {
            network
                .get_mut(&names[1])
                .unwrap()
                .linked_nodes
                .push(names[0].clone());
        }
    }
    let res = crawl_twice(&network, network.get("start").unwrap(), &mut HashMap::new());
    println!("{}", res);

    Ok(())
}

fn crawl_twice(
    network: &HashMap<String, Node>,
    actual_cave: &Node,
    visited_small_caves: &mut HashMap<String, u8>,
) -> i32 {
    // println!("In: {}", actual_cave.id);
    if actual_cave.id == "end" {
        return 1;
    }
    if !actual_cave.is_big {
        *visited_small_caves
            .entry(actual_cave.id.clone())
            .or_insert(0) += 1;
    }
    let mut count = 0;
    for cave in actual_cave.linked_nodes.iter() {
        if cave == "start" {
            continue;
        }
        match visited_small_caves.get(cave) {
            Some(v) => {
                if *v < 2 && !visited_small_caves.values().any(|&val| val == 2) {
                    count += crawl_twice(network, &network[cave], &mut visited_small_caves.clone());
                }
            }
            None => count += crawl_twice(network, &network[cave], &mut visited_small_caves.clone()),
        }
    }
    count
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
