use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_input_lines(path: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(error) => {
            println!("Error opening intput file: {error}");
            return Err(error);
        }
    };

    let reader = BufReader::new(file);
    let mut results: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let report: Vec<u8> = line?
            .split_whitespace()
            .map(|val| val.parse::<u8>().unwrap())
            .collect();

        println!("REPORT: {:?}", report);

        results.push(report);
    }

    Ok(results)
}

fn is_increasing(report: &Vec<u8>) -> bool {
    for idx in 0..report.len() - 1 {
        if report[idx] > report[idx + 1] {
            return false;
        }
    }
    true
}

fn is_decreasing(report: &Vec<u8>) -> bool {
    for idx in 0..report.len() - 1 {
        if report[idx] < report[idx + 1] {
            return false;
        }
    }
    true
}

fn valid_adjetant(report: &Vec<u8>) -> bool {
    let at_most = 3;
    let at_least = 1;

    for idx in 0..report.len() - 1 {
        if report[idx].abs_diff(report[idx + 1]) > at_most
            || report[idx].abs_diff(report[idx + 1]) < at_least
        {
            return false;
        }
    }
    true
}

fn report_is_valid(report: &Vec<u8>) -> bool {
    let is_inc = is_increasing(report);
    let is_dec = is_decreasing(report);
    let valid_adjetant = valid_adjetant(report);

    if !is_dec && !is_inc {
        return false;
    }

    if !valid_adjetant {
        return false;
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 02!");

    let path = "./src/resources/day02input";
    let reports = read_input_lines(path).unwrap();
    let mut results: Vec<bool> = Vec::new();

    for report in reports {
        let result = report_is_valid(&report);

        if result {
            results.push(result);
        }
    }

    let count = results.len();
    println!("COUNT: {count}");

    Ok(())
}
