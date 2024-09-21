use std::{fs, env, io::{self, BufRead}};
use grid::Grid;

#[cfg(test)]
mod tests {
    use super::*;

    
}
fn step(grid: &mut Grid<char>) {
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
    for r in 0..grid.rows() {
        for c in 1..grid.cols() {
            if *grid.get(r, c).unwrap() == 'O' {
                let mut i = c as isize - 1;
                while i >= 0 && *grid.get(r, i).unwrap() == '.' {
                    i -= 1;
                }
                if i + 1 < (c as isize) {
                    *grid.get_mut(r, i + 1).unwrap() = 'O';
                    *grid.get_mut(r, c).unwrap() = '.';
                }
            }
        }
    }
    
    for r in (0..(grid.rows() - 1)).rev() {
        for c in 0..grid.cols() {
            if *grid.get(r, c).unwrap() == 'O' {
                let mut i = r + 1;
                while i < grid.rows() && *grid.get(i, c).unwrap() == '.' {
                    i += 1;
                }
                if i - 1 > r {
                    *grid.get_mut(i - 1, c).unwrap() = 'O';
                    *grid.get_mut(r, c).unwrap() = '.';
                }
            }
        }
    }
    for r in 0..grid.rows() {
        for c in (0..(grid.cols() - 1)).rev() {
            if *grid.get(r, c).unwrap() == 'O' {
                let mut i = c + 1;
                while i < grid.cols() && *grid.get(r, i).unwrap() == '.' {
                    i += 1;
                }
                if i - 1 > c {
                    *grid.get_mut(r, i - 1).unwrap() = 'O';
                    *grid.get_mut(r, c).unwrap() = '.';
                }
            }
        }
    }
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
    let limit = 1_000_000_000;
    let mut n1 = 0;
    let mut n2 = 0;
    let mut board1 = grid.clone();
    let mut board2 = grid.clone();
    loop {
        step(&mut board1);
        n1 += 1;
        step(&mut board2);
        step(&mut board2);
        n2 += 2;
        if board1 == board2 { break; }
    }
    let m = (limit - n2) % (n2 - n1);
    grid = board2;
    for _ in 0..m {
        step(&mut grid);
    }
    for ((r, _), ch) in grid.indexed_iter() {
        if *ch == 'O' {
            result += grid.rows() - r;
        }
    }
    println!("{}", result);
}
