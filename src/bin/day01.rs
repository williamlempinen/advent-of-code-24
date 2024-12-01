use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_file_input_to_tuple_of_vecs(
    filepath: &str,
) -> Result<(Vec<u32>, Vec<u32>), std::io::Error> {
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening file");
            return Err(error);
        }
    };

    let reader = BufReader::new(file);
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        for (idx, val) in line?.split_whitespace().enumerate() {
            match idx % 2 == 0 {
                true => left.push(val.parse::<u32>().unwrap()),
                false => right.push(val.parse::<u32>().unwrap()),
            }
        }
    }

    Ok((left, right))
}

fn calculate_distance(l: &Vec<u32>, r: &Vec<u32>) -> u32 {
    let mut i = 0;
    let end = l.len();
    let mut results = Vec::new();

    while i < end {
        let distance = l[i].abs_diff(r[i]);
        results.push(distance);
        i += 1;
    }

    results.iter().sum()
}

fn main() -> Result<(), String> {
    println!("Day 1!");

    let path = "./src/resources/day01input";
    let vecs = read_file_input_to_tuple_of_vecs(path);

    let (mut left, mut right) = vecs.unwrap();
    left.sort();
    right.sort();

    let distance = calculate_distance(&left, &right);
    println!("DISTANCE: {distance}");

    Ok(())
}
