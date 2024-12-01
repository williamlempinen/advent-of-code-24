use std::{
    env,
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

fn main() -> Result<(), String> {
    println!("Day 1!");
    let current_dir = env::current_dir().expect("error dir");
    println!("{current_dir:?}");
    let path = "./src/resources/day01input";
    let vecs = read_file_input_to_tuple_of_vecs(path);

    let (mut left, mut right) = vecs.unwrap();
    left.sort();
    right.sort();

    println!("LEFT: {left:?}");
    println!("RIGHT: {right:?}");

    let mut results = Vec::new();

    let mut i = 0;
    let end = left.len();

    while i < end {
        let distance = left[i].abs_diff(right[i]);
        println!("DIST: {distance}");
        results.push(distance);
        i += 1;
    }

    let sum: u32 = results.iter().sum();
    println!("SUM: {sum}");

    Ok(())
}
