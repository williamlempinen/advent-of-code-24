use std::{
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

fn is_increasing(report: &Vec<u8>) -> (bool, bool) {
    let mut tolerate = false;
    println!("STARTING POINT: {:?}, tolerate: {}", report, tolerate);
    for idx in 0..report.len() - 1 {
        if report[idx] > report[idx + 1] {
            if tolerate {
                return (false, tolerate);
            } else {
                println!("UPDATING tolerate");
                tolerate = true;
            }
        }
    }
    (true, tolerate)
}

fn is_decreasing(report: &Vec<u8>) -> (bool, bool) {
    let mut tolerate = false;
    println!("STARTING POINT: {:?}, tolerate: {}", report, tolerate);
    for idx in 0..report.len() - 1 {
        if report[idx] < report[idx + 1] {
            if tolerate {
                return (false, tolerate);
            } else {
                println!("UPDATING tolerate");
                tolerate = true;
            }
        }
    }
    (true, tolerate)
}

fn valid_adjetant(report: &Vec<u8>, tolerate: bool) -> bool {
    println!("STARTING POINT: {:?}, tolerate: {}", report, tolerate);
    let mut local_t = false;
    let at_most = 3;
    let at_least = 1;

    for idx in 0..report.len() - 1 {
        if report[idx].abs_diff(report[idx + 1]) > at_most
            || report[idx].abs_diff(report[idx + 1]) < at_least
        {
            if tolerate || local_t {
                println!("NOT VALID ADJENTANT");
                return false;
            } else {
                println!("UPDATING tolerate");
                local_t = true;
            }
        }
    }
    true
}

fn report_is_valid(report: &Vec<u8>) -> bool {
    let (is_inc, inc_tolerate) = is_increasing(report);
    let (is_dec, dec_tolerate) = is_decreasing(report);

    if is_inc {
        return valid_adjetant(report, inc_tolerate);
    }

    if is_dec {
        return valid_adjetant(report, dec_tolerate);
    }

    return false;
}

fn main() -> Result<(), String> {
    println!("Day 02!");

    let path = "./src/resources/day02test";
    let reports = read_input_lines(path).unwrap();
    let mut results: Vec<bool> = Vec::new();

    for report in reports {
        let result = report_is_valid(&report);
        println!("RES: {result}");

        if result {
            results.push(result);
        }
    }

    let count = results.len();
    println!("COUNT: {count}");

    Ok(())
}
