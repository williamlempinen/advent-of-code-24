use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

type Location = (isize, isize);

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

fn read_unique_antennas(matrice: &Vec<Vec<char>>) -> HashMap<char, Vec<Location>> {
    let mut antennas: HashMap<char, Vec<Location>> = HashMap::new();

    for (x_idx, row) in matrice.iter().enumerate() {
        for (y_idx, c) in row.iter().enumerate() {
            let location = (x_idx as isize, y_idx as isize);
            antennas.entry(*c).or_insert_with(Vec::new).push(location);
        }
    }

    antennas
}

fn antinode_points_for_antenna(
    constraints: &(usize, usize),
    antenna_data: &(char, Vec<Location>),
) -> Option<Vec<Location>> {
    if antenna_data.1.len() <= 1 {
        return None;
    }

    let mut antinode_locations: Vec<Location> = Vec::new();
    let map_top = 0;
    let map_bottom = constraints.0;
    let map_left = 0;
    let map_right = constraints.1;

    for (idx, &ant_1) in antenna_data.1.iter().enumerate() {
        for &ant_2 in &antenna_data.1[idx + 1..] {
            let middle: Location = ((ant_1.0 + ant_2.0) / 2, (ant_1.1 + ant_2.1) / 2);

            if (ant_1.0 + ant_2.0) % 2 == 0 && (ant_1.1 + ant_2.1) % 2 == 0 {
                let delta_row = ant_2.0 as isize - middle.0 as isize;
                let delta_col = ant_2.1 as isize - middle.1 as isize;

                let antinode1 = (
                    (middle.0 as isize - delta_row),
                    (middle.1 as isize - delta_col),
                );

                let antinode2 = (
                    (middle.0 as isize + delta_row),
                    (middle.1 as isize + delta_col),
                );

                if antinode1.0 >= map_top
                    && antinode1.0 < map_bottom as isize
                    && antinode1.1 >= map_left
                    && antinode1.1 < map_right as isize
                {
                    antinode_locations.push(antinode1);
                }

                if antinode2.0 >= map_top
                    && antinode2.0 < map_bottom as isize
                    && antinode2.1 >= map_left
                    && antinode2.1 < map_right as isize
                {
                    antinode_locations.push(antinode2);
                }
            }
        }

        if antinode_locations.is_empty() {
            return None;
        }
    }

    return Some(antinode_locations);
}

fn flat_and_remove_duplicates(locations: Vec<Vec<Location>>) -> Vec<Location> {
    let mut set = HashSet::new();
    let res = locations
        .into_iter()
        .flat_map(|l| l)
        .filter(|l| set.insert(*l))
        .collect::<Vec<Location>>();
    res
}

fn main() {
    println!("Day 8!");

    // not done / not correct

    let path = "./src/inputs/day08input";

    let matrice = read_input_to_matrice(path).expect("Error reading input");

    println!("MATRICE:  {matrice:?}");

    let antenna_data = read_unique_antennas(&matrice);

    println!("MAP: {antenna_data:?}");

    let contraints = (matrice.len(), matrice[0].len());

    let mut results: Vec<Vec<Location>> = Vec::new();

    for antenna in antenna_data {
        match antinode_points_for_antenna(&contraints, &antenna) {
            Some(a) => results.push(a.to_owned()),
            None => {
                println!("NO ANTINODES");
            }
        };
    }

    let antinodes = flat_and_remove_duplicates(results);

    println!("ANTINODES: {antinodes:?}");

    let sum = antinodes.iter().count();
    println!("RESULT: {sum}");
}
