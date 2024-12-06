use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_rules_and_updates(path: &str) -> Result<(Vec<(i32, i32)>, Vec<Vec<i32>>), io::Error> {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut is_reading_rules = true;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            is_reading_rules = false;
        }

        if is_reading_rules {
            let rule: Vec<&str> = line.split("|").collect();

            rules.push((
                rule[0].parse::<i32>().unwrap(),
                rule[1].parse::<i32>().unwrap(),
            ));
        } else {
            if line.is_empty() {
                continue;
            }

            let mut update: Vec<i32> = Vec::new();

            for num in line.split(",").into_iter() {
                update.push(num.parse::<i32>().unwrap());
            }

            updates.push(update);
        }
    }

    Ok((rules, updates))
}

fn parse_update(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for &(first, second) in rules {
        let first_idx = update.iter().position(|&page| page == first);
        let second_idx = update.iter().position(|&page| page == second);

        if first_idx.is_some() && second_idx.is_some() {
            if first_idx.unwrap() > second_idx.unwrap() {
                return false;
            }
        }
    }

    true
}

fn fix_incorrect_update_and_give_middle(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Option<i32> {
    let mut fix_update = update.clone();

    fix_update.sort_by(|&a, &b| {
        for &(first, second) in rules {
            if a == first && b == second {
                return Less;
            }
            if a == second && b == first {
                return Greater;
            }
        }
        Equal
    });

    return find_middle_from_update(&fix_update);
}

fn find_middle_from_update(update: &Vec<i32>) -> Option<i32> {
    if update.len() % 2 == 0 {
        return update.get(update.len() / 2 - 1).copied();
    }

    return update.get(update.len() / 2).copied();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 5!");

    let path = "./src/resources/day05input";
    let (rules, updates) = read_rules_and_updates(path)?;

    let mut middles_sum = 0;

    for update in updates {
        let is_valid = parse_update(&update, &rules);

        if !is_valid {
            middles_sum += fix_incorrect_update_and_give_middle(&update, &rules).unwrap();
        }
    }

    println!("MIDDLES SUM: {middles_sum}");

    Ok(())
}
