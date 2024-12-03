use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

use regex::{Regex, RegexSet};

fn read_input_to_string(filepath: &str) -> Result<String, io::Error> {
    let mut file = File::open(filepath).expect("Error opening input file");
    let mut input = String::new();

    file.read_to_string(&mut input)
        .expect("Error reading file into string");

    Ok(input)
}

fn extract_valid_commands(string: &String) -> Result<Vec<&str>, Box<dyn Error>> {
    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)|don\'t\(\)|do\(\)").expect("Invalid regex");
    let matches: Vec<&str> = regex.find_iter(string).map(|m| m.as_str()).collect();

    println!("MATCHES: {:?}", matches);

    Ok(matches)
}

fn calculate_commands(command: &str) -> i32 {
    let regex = Regex::new(r"(\d+),(\d+)").expect("Invalid regex");
    let mut ans: i32 = 0;
    let numbers = regex.find_iter(command).map(|x| x.as_str()).into_iter();

    for n in numbers {
        let s: Vec<_> = n.split(',').collect();
        ans = s[0].parse::<i32>().unwrap() * s[1].parse::<i32>().unwrap();
    }

    return ans;
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 3!");

    let path = "./src/resources/day03input";
    let as_string = read_input_to_string(path).unwrap();
    let mut numbers: Vec<i32> = Vec::new();
    let mut temp_stack: Vec<&str> = Vec::new();

    let matches = extract_valid_commands(&as_string)?;

    for command in matches {
        if temp_stack.len() == 0 {
            println!("CALCULATING");
            let temp = calculate_commands(command);
            numbers.push(temp);
        }

        if command == "don't()" && temp_stack.len() == 0 {
            println!("RECEIVED DONT");
            temp_stack.push("no");
        }

        if command == "do()" && temp_stack.len() == 1 {
            println!("RECEIVED DO()");
            temp_stack.pop();
        }
    }

    let total: i32 = numbers.iter().sum();

    println!("TOTAL: {}", total);

    Ok(())
}
