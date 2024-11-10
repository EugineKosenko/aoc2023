use std::{fs, env, io::{self, BufRead}};
use grid::Grid;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::cmp;
use std::collections::HashMap;

type Board = Grid<usize>;
type Pos = (usize, usize);
#[derive(EnumIter, PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir { North, East, South, West }
type Path = (Vec<Pos>, usize, Dir, usize);
type Paths = Vec<Path>;
type Weight = (cmp::Reverse<usize>, cmp::Reverse<usize>);

fn weight(board: &Board, path: &Path) -> Weight {
    (cmp::Reverse(distance(path.0.last().unwrap(), &(board.rows() - 1, board.cols() - 1))),
     cmp::Reverse(path.3))
}
fn distance(p1: &Pos, p2: &Pos) -> usize {
    ((p1.0 as isize - p2.0 as isize).abs()
        + (p1.1 as isize - p2.1 as isize).abs()) as usize
}
fn show(rows: usize, cols: usize, path: &Vec<Pos>) -> Grid<char> {
    let mut result = Grid::new(rows, cols);
    result.fill('.');
    for i in 1..path.len() {
        let (rin, cin) = path[i-1];
        let (rout, cout) = path[i];
        *result.get_mut(rout, cout).unwrap() =
            if rin < rout {
                'v'
            } else if rin > rout {
                '^'
            } else if cin < cout {
                '>'
            } else if cin > cout {
                '<'
            } else {
                '?'
            };
    }
    result
}
#[derive(PartialEq, Eq, Hash)]
struct Key(Pos, Dir, usize);

type Limits = HashMap<Key, usize>;
fn dirs(d: Dir) -> [Dir; 3] {
    match d {
        Dir::North => [Dir::North, Dir::East, Dir::West],
        Dir::East => [Dir::North, Dir::East, Dir::South],
        Dir::South => [Dir::East, Dir::South, Dir::West],
        Dir::West => [Dir::North, Dir::South, Dir::West]
    }
}
fn read_board(name: &str) -> Board {
    let file = fs::File::open(name).unwrap();
    let lines = io::BufReader::new(file)
        .lines().map(|l| l.unwrap());
    let mut board = Board::new(0, 0);
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as usize);
        }
        board.push_row(row);
    }
    board
}

fn cost(board: &Board, path: &Vec<Pos>) -> usize {
    path.iter().skip(1).fold(0, |a, (r, c)| a + board.get(*r, *c).unwrap())
}

#[test]
fn test_optimal() {
    let board = read_board("day17_debug.txt");
    let opt = vec![
        (0, 0), (0, 1), (0, 2),
        (1, 2), (1, 3), (1, 4), (1, 5),
        (0, 5), (0, 6), (0, 7), (0, 8),
        (1, 8),
        (2, 8), (2, 9), (2, 10),
        (3, 10), (4, 10),
        (4, 11), (5, 11), (6, 11), (7, 11),
        (7, 12), (8, 12), (9, 12), (10, 12),
        (10, 11), (11, 11), (12, 11), (12, 12)
    ];
    println!("{:#?}", show(13, 13, &opt));
    assert_eq!(cost(&board, &opt), 102);
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
            row.push(c.to_digit(10).unwrap() as usize);
        }
        board.push_row(row);
    }
    let (rows, cols) = (board.rows(), board.cols());
    println!("{:#?}", board);
    let mut paths: Paths = Paths::with_capacity(1000);
    
    for path in vec![
        (vec![(0, 0), (0, 1)], 1, Dir::East, *board.get(0, 1).unwrap()),
        (vec![(0, 0), (1, 0)], 1, Dir::South, *board.get(1, 0).unwrap())
    ] {
        let i = match paths.binary_search_by_key(&weight(&board, &path), |p| { weight(&board, p) }) {
            Err(i) => i,
            Ok(i) => i + 1
        };
        paths.insert(i, path);
    }
    let mut limits = Limits::new();
    limits.insert(Key((0, 1), Dir::East, 2), *board.get(0, 1).unwrap());
    limits.insert(Key((1, 0), Dir::East, 2), *board.get(1, 0).unwrap());
    let mut limit = usize::MAX;
    let mut k = 0;
    while let Some((path, rest, din, cost)) = paths.pop() {
        k += 1;
        if k % 1_000_000 == 0 {
            println!("{} {} {}", k, limit, paths.len());
            // println!("{:?} {} {:?}, {}", path.clone(), len, din, w);
        }
        let &pos = path.last().unwrap();
        if pos == (rows - 1, cols - 1) {
            limit = limit.min(cost);
            println!("{}", limit);
            // println!("{:#?}", show(rows, cols, &path));
            continue;
        }
        for dout in dirs(din) {
            if rest == 0 && dout == din { continue; }
            let (mut r, mut c) = pos;
            match dout {
                Dir::North => { r = if r == 0 { continue; } else { r - 1 }; },
                Dir::East => { c = if c == cols - 1 { continue; } else { c + 1 }; },
                Dir::South => { r = if r == rows - 1 { continue; } else { r + 1 }; },
                Dir::West => { c = if c == 0 { continue; } else { c - 1 }; }
            };
            if cost + board.get(r, c).unwrap() >= limit { continue; }
            let rest = if dout == din { rest - 1 } else { 2 };
            let key = Key((r, c), dout, rest);
            if cost + board.get(r, c).unwrap() >= *limits.get(&key).unwrap_or(&usize::MAX) {
                continue;
            } else {
                limits.insert(key, cost + board.get(r, c).unwrap());
            }
            // if path.contains(&(r, c)) { continue; }
            let mut path = path.clone();
            path.push((r, c));
            let path = (path, rest, dout, cost + board.get(r, c).unwrap());
            let i = match paths.binary_search_by_key(&weight(&board, &path), |p| weight(&board, p)) {
                Err(i) => i,
                Ok(i) => i + 1
            };
            paths.insert(i, path);
        }
    }
    let result = limit;
    println!("{:?}", result);
}
