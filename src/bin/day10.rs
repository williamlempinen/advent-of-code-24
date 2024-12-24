use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_input_to_matrice(path: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut matrice: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
        matrice.push(row);
    }

    Ok(matrice)
}

fn find_trailhead_scores(m: Vec<Vec<u8>>) -> u16 {
    let rows = m.len();
    let cols = m[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut total_score = 0;

    for r in 0..rows {
        for c in 0..cols {
            if m[r][c] == 0 {
                let mut visited = HashSet::new();
                let mut queue = VecDeque::new();
                let mut reachable_nines = 0;

                queue.push_back((r, c, 0));
                visited.insert((r, c));

                while let Some((x, y, height)) = queue.pop_front() {
                    for (dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && ny >= 0 && (nx as usize) < rows && (ny as usize) < cols {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            let next_height = m[nx][ny];

                            if !visited.contains(&(nx, ny)) && next_height == height + 1 {
                                visited.insert((nx, ny));
                                queue.push_back((nx, ny, next_height));

                                if next_height == 9 {
                                    reachable_nines += 1;
                                }
                            }
                        }
                    }
                }

                total_score += reachable_nines;
            }
        }
    }

    total_score
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 10!");

    let path = "./src/inputs/day10input";

    let m = read_input_to_matrice(path)?;
    println!("Matrice: {m:?}");

    let score = find_trailhead_scores(m);
    println!("SCORE: {score}");

    Ok(())
}
