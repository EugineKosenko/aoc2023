use std::{fs, env, io::{self, BufRead}};
use grid::Grid;

type Board = Grid<char>;
fn is_internal(right: isize, (row, col): &(isize, isize), circuit: &Vec<(isize, isize)>) -> bool {
    let mut crosses_count = 0;
    let mut i = col + 1;
    while i <= right {
        if !circuit.contains(&(*row, i)) {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j <= right && circuit.contains(&(*row, j)) { j += 1; }
        if !circuit.contains(&(*row, j)) { j -= 1; }
        if circuit.contains(&(row - 1, i)) && circuit.contains(&(row + 1, j)) ||
            circuit.contains(&(row + 1, i)) && circuit.contains(&(row - 1, j)) {
                crosses_count += 1;
            }
        i = j + 1;
        i += 1;
    }
    crosses_count % 2 == 1
}

fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines().map(|line| line.unwrap());
    let (mut row, mut col) = (0, 0);
    let mut circuit = vec![];
    for line in lines {
        let mut items = line.split_whitespace();
        let dir = items.next().unwrap().chars().next().unwrap();
        let dist = items.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..dist {
            match dir {
                'U' => { row -= 1; },
                'R' => { col += 1; },
                'D' => { row += 1; },
                'L' => { col -= 1; },
                _ => panic!("Invalid dir")
            };
            circuit.push((row, col));
        }
    }
    let top = circuit.iter().map(|p| p.0).min().unwrap();
    let bottom = circuit.iter().map(|p| p.0).max().unwrap();
    let left = circuit.iter().map(|p| p.1).min().unwrap();
    let right = circuit.iter().map(|p| p.1).max().unwrap();
    let mut board = Board::init((bottom - top + 1) as usize, (right - left + 1) as usize, '.');
    for (row, col) in &circuit {
        *board.get_mut(row - top, col - left).unwrap() = '#';
    }
    for row in (top+1)..bottom {
        eprintln!("Row {}", row);
        for col in (left+1)..right {
            if !circuit.contains(&(row, col)) {
                if is_internal(right, &(row, col), &circuit) {
                    result += 1;
                    *board.get_mut(row - top, col - left).unwrap() = '#';
                } else {
                    //*board.get_mut(row - top, col - left).unwrap() = '0';
                }
            }
        }
    }
    result += circuit.len();
    eprintln!("{:?}", result);
}
