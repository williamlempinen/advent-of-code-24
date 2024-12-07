use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_input_vec(path: &str) -> Result<Vec<(i64, Vec<i64>)>, io::Error> {
    let mut results: Vec<(i64, Vec<i64>)> = Vec::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(":").collect();
        let operation: (i64, Vec<i64>);
        let mut numbers: Vec<i64> = Vec::new();

        for item in parts[1].split_whitespace() {
            println!("ITEM: {item}");
            numbers.push(item.parse::<i64>().unwrap());
        }

        operation = (parts[0].trim().parse::<i64>().unwrap(), numbers);
        results.push(operation);
    }

    Ok(results)
}

fn validate_operation(operation: &(i64, Vec<i64>)) -> Option<i64> {
    let operators = vec!['+', '*', '|'];
    let num_operators = operation.1.len() - 1;
    let total_combinations = operators.len().pow(num_operators as u32);

    for i in 0..total_combinations {
        let mut operator_combo = Vec::new();
        let mut current = i;

        for _ in 0..num_operators {
            operator_combo.push(operators[current % operators.len()]);
            current /= operators.len();
        }

        let mut result = operation.1[0];
        let mut expression = format!("{}", operation.1[0]);

        for (j, &operator) in operator_combo.iter().enumerate() {
            match operator {
                '+' => result += operation.1[j + 1],
                '*' => result *= operation.1[j + 1],
                '|' => {
                    result = format!("{}{}", result, operation.1[j + 1])
                        .parse::<i64>()
                        .unwrap()
                }
                _ => panic!("Error parsing operators"),
            }

            expression.push(operator);
            expression.push_str(&operation.1[j + 1].to_string());
        }

        if result == operation.0 {
            return Some(result);
        }
    }

    None
}

fn main() {
    println!("Day 7!");

    let path = "./src/inputs/day07input";

    let inputs = read_input_vec(path).expect("Error parsing inputs");
    println!("INPUTS: {inputs:?}");
    let mut results: Vec<i64> = Vec::new();

    for input in inputs {
        match validate_operation(&input) {
            Some(ans) => results.push(ans),
            None => {
                println!("NOT VALID")
            }
        };
    }

    let ans: i64 = results.iter().sum();
    println!("SUM: {ans}");
}
