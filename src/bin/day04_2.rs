use std::{
    fs::File,
    io::{BufRead, BufReader},
    isize,
};

fn read_input_to_char_matrice(path: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(path)?;
    let mut matrice: Vec<Vec<char>> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let string = line?;
        let mut temp: Vec<char> = Vec::new();

        for char in string.chars() {
            temp.push(char);
        }

        matrice.push(temp);
    }

    Ok(matrice)
}

fn is_mas_or_sam(pattern: &Vec<char>) -> bool {
    if pattern.len() > 3 {
        return false;
    }

    if pattern[0] == 'M' && pattern[1] == 'A' && pattern[2] == 'S'
        || pattern[0] == 'S' && pattern[1] == 'A' && pattern[2] == 'M'
    {
        return true;
    }
    false
}

fn search_x_mas(matrice: &Vec<Vec<char>>) -> Result<i32, anyhow::Error> {
    let matrice_len = matrice.len();
    let row_len = matrice[0].len();

    let mut count = 0;

    for row_idx in 1..matrice_len - 1 {
        for char_idx in 1..row_len - 1 {
            let right_pattern = vec![
                matrice[row_idx - 1][char_idx - 1],
                matrice[row_idx][char_idx],
                matrice[row_idx + 1][char_idx + 1],
            ];

            let left_pattern = vec![
                matrice[row_idx + 1][char_idx - 1],
                matrice[row_idx][char_idx],
                matrice[row_idx - 1][char_idx + 1],
            ];

            if is_mas_or_sam(&right_pattern) && is_mas_or_sam(&left_pattern) {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 4!");
    let path = "./src/resources/day04input";

    let word_matrice = read_input_to_char_matrice(path)?;

    let mas_count = search_x_mas(&word_matrice).expect("Error counting mas");

    println!("RESULTS: {}", mas_count);

    Ok(())
}
