use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

enum Action {
    ReplaceWithOne,
    Split,
    Multiply2024,
}

impl Action {
    fn get_action(n: &u64) -> Self {
        if *n == 0 {
            return Self::ReplaceWithOne;
        } else if n.to_string().len() % 2 == 0 {
            return Self::Split;
        } else {
            return Self::Multiply2024;
        }
    }
}

fn read_to_vec(path: &str) -> Result<Vec<u64>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut nums: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        for num in line.split_whitespace() {
            let n = num.parse::<u64>();
            nums.push(n.unwrap());
        }
    }

    Ok(nums)
}

fn run_rules(num: u64, cache: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    if let Some(result) = cache.get(&num) {
        return result.clone();
    }

    let result = match Action::get_action(&num) {
        Action::ReplaceWithOne => {
            vec![1]
        }
        Action::Multiply2024 => {
            vec![num * 2024]
        }
        Action::Split => {
            let as_str = num.to_string();
            let mid = as_str.len() / 2;
            let l = as_str[..mid].parse::<u64>().unwrap();
            let r = as_str[mid..].parse::<u64>().unwrap();
            vec![l, r]
        }
    };

    cache.insert(num, result.clone());

    result
}

fn main() {
    println!("Day 11!");
    let path = "./src/inputs/day11input";
    let mut nums = read_to_vec(path).unwrap();
    let mut cache = HashMap::new();

    for _blink in 0..75 {
        println!("BLINK");
        let mut temp: Vec<Vec<u64>> = Vec::new();

        for n in nums {
            let res = run_rules(n, &mut cache);
            temp.push(res);
        }
        nums = temp.into_iter().flatten().collect();
    }

    println!("LEN: {}", nums.len());
}
