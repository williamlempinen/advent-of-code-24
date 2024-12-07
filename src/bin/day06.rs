use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

enum Walk {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Walk {
    fn turn(&self) -> Self {
        let new_direction = match &self {
            Walk::UP => Walk::RIGHT,
            Walk::RIGHT => Walk::DOWN,
            Walk::DOWN => Walk::LEFT,
            Walk::LEFT => Walk::UP,
        };

        new_direction
    }
}

fn read_input_to_matrice(path: &str) -> Result<Vec<Vec<char>>, io::Error> {
    let mut matrice: Vec<Vec<char>> = Vec::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut row: Vec<char> = Vec::new();

        for c in line.chars() {
            row.push(c);
        }

        matrice.push(row);
    }

    Ok(matrice)
}

fn walk(
    current_point: &(usize, usize),
    map: &Vec<Vec<char>>,
    direction: &Walk,
) -> Option<(usize, usize)> {
    let new_point = current_point.clone();
    let new_map = map.clone();

    match direction {
        Walk::UP => {
            println!("Walking up");
            println!("POINT: {new_point:?}");
            if new_point.0 > 0 && new_map[new_point.0 - 1][new_point.1] != '#' {
                return Some((new_point.0 - 1, new_point.1));
            }
        }
        Walk::DOWN => {
            println!("Walking down");
            println!("POINT: {new_point:?}");
            if new_point.0 + 1 < map.len() && map[new_point.0 + 1][new_point.1] != '#' {
                return Some((new_point.0 + 1, new_point.1));
            }
        }
        Walk::LEFT => {
            println!("Walking left");
            println!("POINT: {new_point:?}");
            if new_point.1 > 0 && map[new_point.0][new_point.1 - 1] != '#' {
                return Some((new_point.0, new_point.1 - 1));
            }
        }
        Walk::RIGHT => {
            println!("Walking right");
            println!("POINT: {new_point:?}");
            if new_point.1 + 1 < map[new_point.0].len() && map[new_point.0][new_point.1 + 1] != '#'
            {
                return Some((new_point.0, new_point.1 + 1));
            }
        }
    }

    return None;
}

fn map_distinct(map: &Vec<Vec<char>>) -> usize {
    let mut starting_point = (0, 0);
    let mut current_direction = Walk::UP;
    let mut distinct_map: HashSet<(usize, usize)> = HashSet::new();

    for (row_idx, row) in map.iter().enumerate() {
        for (c_idx, c) in row.iter().enumerate() {
            if *c == '^' {
                starting_point = (row_idx, c_idx);
                break;
            }
        }
    }

    distinct_map.insert(starting_point);

    loop {
        println!("Start loop");

        if let Some(new_point) = walk(&starting_point, &map, &current_direction) {
            starting_point = new_point;
            distinct_map.insert(starting_point);
        } else {
            current_direction = Walk::turn(&current_direction);
        }

        if distinct_map.len() >= map.len() * map[0].len() {
            break;
        }

        if starting_point.0 >= map.len() || starting_point.1 >= map[0].len() {
            break;
        }
    }

    return distinct_map.len();
}

fn main() {
    println!("Day 6!");

    // not done / not correct

    let path = "./src/inputs/day06input";
    let map = read_input_to_matrice(path).expect("Error reading map");
    let result = map_distinct(&map);

    println!("RESULTS: {result}");
}
