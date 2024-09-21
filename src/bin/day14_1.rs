use std::{fs, env, io::{self, BufRead}};
use grid::Grid;

#[cfg(test)]
mod tests {
    use super::*;

    
}

fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines().map(|l| l.unwrap());
    let mut grid = Grid::new(0, 0);
    for line in lines {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        grid.push_row(row);
    }
    for r in 1..grid.rows() {
        for c in 0..grid.cols() {
            if *grid.get(r, c).unwrap() == 'O' {
                let mut i = r as isize - 1;
                while i >= 0 && *grid.get(i, c).unwrap() == '.' {
                    i -= 1;
                }
                if i + 1 < (r as isize) {
                    *grid.get_mut(i + 1, c).unwrap() = 'O';
                    *grid.get_mut(r, c).unwrap() = '.';
                }
            }
        }
    }
    for ((r, _), ch) in grid.indexed_iter() {
        if *ch == 'O' {
            result += grid.rows() - r;
        }
    }
    println!("{}", result);
}
