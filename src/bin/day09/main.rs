use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const BOUND: usize = 100;

fn ex1() -> Result<(), Error> {
    let input = File::open("./src/bin/day09/input.txt")?;
    let buffered = BufReader::new(input);

    let mut smoke_bas: Vec<Vec<u8>> = vec![];

    for line in buffered.lines() {
        let values = line?;
        smoke_bas.push(
            values
                .chars()
                .map(|value| value.to_digit(10).unwrap() as u8)
                .collect::<_>(),
        );
    }

    let mut low_points = vec![];
    let adj_coors = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut res: u32 = 0;
    for i in 0..smoke_bas.len() {
        for j in 0..BOUND {
            let mut count = 0;
            let mut edges = 0;
            for coors in adj_coors.iter() {
                let (a, b) = coors;
                if (i as i32 + a) >= 0
                    && (i as i32 + a) < smoke_bas.len() as i32
                    && (j as i32 + b) >= 0
                    && (j as i32 + b) < BOUND as i32
                {
                    edges += 1;
                    if smoke_bas[i][j]
                        < smoke_bas[(i as i32 + *a) as usize][(j as i32 + *b) as usize]
                    {
                        count += 1;
                    }
                }
            }
            if count == edges {
                low_points.push((i, j));
                res += 1 + smoke_bas[i][j] as u32;
            }
        }
    }
    println!("{}", res);

    Ok(())
}

fn ex2() -> Result<(), Error> {
    let input = File::open("./src/bin/day09/input.txt")?;
    let buffered = BufReader::new(input);

    let mut smoke_bas: Vec<Vec<u8>> = vec![];

    for line in buffered.lines() {
        let values = line?;
        smoke_bas.push(
            values
                .chars()
                .map(|value| value.to_digit(10).unwrap() as u8)
                .collect::<_>(),
        );
    }

    let mut low_points = vec![];
    let adj_coors = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for i in 0..smoke_bas.len() {
        for j in 0..BOUND {
            let mut count = 0;
            let mut edges = 0;
            for coors in adj_coors.iter() {
                let (a, b) = coors;
                if (i as i32 + a) >= 0
                    && (i as i32 + a) < smoke_bas.len() as i32
                    && (j as i32 + b) >= 0
                    && (j as i32 + b) < BOUND as i32
                {
                    edges += 1;
                    if smoke_bas[i][j]
                        < smoke_bas[(i as i32 + *a) as usize][(j as i32 + *b) as usize]
                    {
                        count += 1;
                    }
                }
            }
            if count == edges {
                low_points.push((i, j));
            }
        }
    }
    let mut biggest_bassins: [usize; 3] = [0; 3];
    for low_point in low_points {
        let found_size = calc_bassin_size(&smoke_bas, low_point.0 as i32, low_point.1 as i32);
        for i in (0..biggest_bassins.len()).rev() {
            if found_size > biggest_bassins[i] {
                biggest_bassins[i] = found_size;
                biggest_bassins.sort();
                biggest_bassins.reverse();
                break;
            }
        }
    }
    println!("{:?}", biggest_bassins.iter().fold(1, |acc, x| acc * x));

    Ok(())
}

fn calc_bassin_size(map: &Vec<Vec<u8>>, low_x: i32, low_y: i32) -> usize {
    let mut bassin: Vec<(i32, i32)> = vec![];
    let mut visited_coors: HashSet<(i32, i32)> = HashSet::new();
    let adj_coors_dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut deq = VecDeque::new();
    deq.push_back((low_x, low_y));

    while !deq.is_empty() {
        let (point_x, point_y) = deq.pop_front().unwrap();
        if visited_coors.contains(&(point_x, point_y)) {
            continue;
        } else {
            visited_coors.insert((point_x, point_y));
            if map[point_x as usize][point_y as usize] < 9 {
                bassin.push((point_x, point_y));
                for coors in adj_coors_dir.iter() {
                    let (a, b) = coors;
                    if (point_x + a) >= 0
                        && (point_x + a) < map.len() as i32
                        && (point_y + b) >= 0
                        && (point_y + b) < BOUND as i32
                    {
                        deq.push_back((point_x + a, point_y + b));
                    }
                }
            }
        }
    }
    bassin.len()
}

fn main() {
    ex1().unwrap();
    ex2().unwrap();
}
