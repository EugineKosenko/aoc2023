use std::{fs, env, io::{self, BufRead}};
use grid::Grid;
use std::collections::BTreeSet;

type Board = Grid<char>;
fn count(board: &Board, beam: Beam) -> usize {
    let rows = board.rows();
    let cols = board.rows();
    let mut visited = Visited::new(rows, cols);
    visited.fill(BTreeSet::new());
    let mut beams = vec![beam];
    while let Some((rin, cin, din)) = beams.pop() {
        if rin < rows && cin < cols && !visited.get(rin, cin).unwrap().contains(&din) {
            visited.get_mut(rin, cin).unwrap().insert(din);
            let dout = match *board.get(rin, cin).unwrap() {
                '.' => din,
                '/' => {
                    match din {
                        Dir::Up => Dir::Right,
                        Dir::Right => Dir::Up,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Down
                    }
                },
                '\\' => {
                    match din {
                        Dir::Up => Dir::Left,
                        Dir::Right => Dir::Down,
                        Dir::Down => Dir::Right,
                        Dir::Left => Dir::Up
                    }
                },
                '-' => {
                    match din {
                        Dir::Up | Dir::Down => { beams.push((rin, cin, Dir::Left)); Dir::Right },
                        d => d
                    }
                },
                '|' => {
                    match din {
                        Dir::Right | Dir::Left => { beams.push((rin, cin, Dir::Down)); Dir::Up },
                        d => d
                    }
                },
                c => { panic!("Invalid tile {}", c); }
            };
            let (rout, cout) = match dout {
                Dir::Up => (if rin > 0 { rin - 1 } else { rows }, cin),
                Dir::Right => (rin, cin + 1),
                Dir::Down => (rin + 1, cin),
                Dir::Left => (rin, if cin > 0 { cin - 1 } else { cols })
            };
            beams.push((rout, cout, dout));
        }
    }
    energized(&visited).iter().filter(|c| **c == '#').count()
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Dir { Up, Right, Down, Left }
type Dirs = BTreeSet<Dir>;
type Visited = Grid<Dirs>;
type Beam = (usize, usize, Dir);
type _Beams = Vec<Beam>;
type Energized = Grid<char>;
fn energized(visited: &Visited) -> Energized {
    let mut result = Energized::new(visited.rows(), visited.cols());
    for ((r, c), v) in visited.indexed_iter() {
        *result.get_mut(r, c).unwrap() = if v.is_empty() { '.' } else { '#' };
    }
    result
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines().map(|l| l.unwrap());
    let mut board = Board::new(0, 0);
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        board.push_row(row);
    }
    let result = (0..board.cols())
        .map(|c| {
            count(&board, (0, c, Dir::Down))
        })
        .max().unwrap();
    let result = (0..board.rows())
        .map(|r| {
            count(&board, (r, board.cols() - 1, Dir::Left))
        })
        .max().unwrap().max(result);
    let result = (0..board.cols())
        .map(|c| {
            count(&board, (board.rows() - 1, c, Dir::Up))
        })
        .max().unwrap().max(result);
    let result = (0..board.rows())
        .map(|r| {
            count(&board, (r, 0, Dir::Right))
        })
        .max().unwrap().max(result);
    println!("{}", result);
}
