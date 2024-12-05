use std::{
    fs::File,
    io::{BufRead, BufReader},
    isize,
};

use anyhow::Result;

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

fn search_word(matrice: &Vec<Vec<char>>, word: String) -> Result<i32, anyhow::Error> {
    let word_as_chars: Vec<char> = word.chars().collect();
    let word_len = word_as_chars.len();

    let matrice_len = matrice.len();
    let row_len = matrice[0].len();

    let mut count = 0;

    let directions: Vec<(isize, isize)> = vec![
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];

    for row_idx in 0..matrice_len {
        for char_idx in 0..row_len {
            for direction in &directions {
                let mut is_match = true;

                for letter_idx in 0..word_len {
                    let current_row = row_idx as isize + direction.0 * letter_idx as isize;
                    let current_col = char_idx as isize + direction.1 * letter_idx as isize;

                    if current_row < 0
                        || current_col < 0
                        || current_row >= matrice_len as isize
                        || current_col >= row_len as isize
                    {
                        is_match = false;
                        break;
                    }

                    if matrice[current_row as usize][current_col as usize]
                        != word_as_chars[letter_idx]
                    {
                        is_match = false;
                        break;
                    }
                }

                if is_match {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn main() -> Result<()> {
    println!("Day 4!");
    let path = "./src/resources/day04input";
    let word = "XMAS".to_string();

    let word_matrice = read_input_to_char_matrice(path)?;
    println!("MATRICES: {:?}", word_matrice);

    let count = search_word(&word_matrice, word).expect("Error counting");

    println!("RESULTS: {}", count);

    Ok(())
}
