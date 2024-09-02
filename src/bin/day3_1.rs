use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;



fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Vec<_>>();
    let mut grid = grid::Grid::new(0, 0);
    for line in lines {
        grid.push_row(line.unwrap().chars().collect());
    }
    for i in 0..grid.rows() {
        let mut j1 = 0;
        while j1 < grid.cols() {
            if grid.get(i, j1).unwrap().is_digit(10) {
                let mut n = String::new();
                let mut j2 = j1;
                while j2 < grid.cols() && grid.get(i, j2).unwrap().is_digit(10) {
                    n.push(*grid.get(i, j2).unwrap());
                    j2 += 1;
                }
                let n = n.parse::<usize>().unwrap();
                let mut is_adjacent = false;
                for k in (i.max(1)-1)..(grid.rows().min(i+2)) {
                    for l in (j1.max(1)-1)..(grid.cols().min(j2+1)) {
                        let c = *grid.get(k, l).unwrap();
                        if !c.is_digit(10) && c != '.' {
                            is_adjacent = true;
                            break;
                        }
                    }
                }
                if is_adjacent {
                    result += n;
                }
                j1 = j2 - 1;
            }
            j1 += 1;
        }
    }
    println!("{}", result);
}
