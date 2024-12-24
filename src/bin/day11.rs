use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

enum Action {
    ReplaceWithOne,
    Split,
    Multiply2024,
}

impl Action {
    fn get_action(n: &u64) -> Action {
        if *n == 0 {
            return Action::ReplaceWithOne;
        } else if n.to_string().len() % 2 == 0 {
            return Action::Split;
        } else {
            return Action::Multiply2024;
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

fn run_rules(num: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    println!("NUM: {num}");
    match Action::get_action(&num) {
        Action::ReplaceWithOne => {
            res.push(1);
        }
        Action::Multiply2024 => {
            let n = num * 2024;
            res.push(n);
        }
        Action::Split => {
            let as_str = num.to_string();
            let mid = as_str.len() / 2;
            let l = as_str[..mid].parse::<u64>().unwrap();
            let r = as_str[mid..].parse::<u64>().unwrap();
            res.push(l);
            res.push(r);
        }
    };

    println!("RUN RULES RES: {res:?}");

    res
}

fn main() {
    println!("Day 11!");
    let path = "./src/inputs/day11input";
    let mut nums = read_to_vec(path).unwrap();
    println!("VEC: {nums:?}");

    for _blink in 0..25 {
        println!("BLINK");
        let mut temp: Vec<Vec<u64>> = Vec::new();

        for n in nums {
            let res = run_rules(n);
            temp.push(res);
        }
        nums = temp.into_iter().flatten().collect();
        //println!("UPDATED: NUMS: {nums:?}");
    }

    println!("LEN: {}", nums.len());
}
