use std::{fs, env, io::{self, BufRead}};
use grid::Grid;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::cmp;

type Board = Grid<usize>;
type Pos = (usize, usize);
#[derive(EnumIter, PartialEq, Clone, Copy, Debug)]
enum Dir { North, East, South, West }
type Path = (Vec<Pos>, usize, Dir, usize);
type Paths = Vec<Path>;
type Weight = (usize, cmp::Reverse<usize>, cmp::Reverse<usize>);

fn weight(board: &Board, path: &Path) -> Weight {
    (path.0.len(),
     cmp::Reverse(distance(path.0.last().unwrap(), &(board.rows() - 1, board.cols() - 1))),
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
fn mincost(
    board: &Board, path: &Vec<Pos>, pos: Pos, rest: usize, din: Dir, cost: usize, limit: usize, depth: usize, step: &mut usize
) -> Option<usize> {
    *step += 1;
    if *step % 10_000_000 == 0 { println!("{} {} {}", *step, limit, usize::MAX - depth); }
    if depth == 0 {
        //println!("{} {:?}", cost, path);
        return Some(cost);
    }
    let (rows, cols) = (board.rows(), board.cols());
    let cost = cost + *board.get(pos.0, pos.1).unwrap();
    if pos == (rows - 1, cols - 1) {
        println!("{} {}", limit, cost);
        //println!("{:#?}", show(rows, cols, &path));
        return Some(cost);
    }
    let mut path = path.clone();
    path.push(pos);
    let mut variants = Dir::iter()
        .filter_map(|dout| {
            if rest == 0 && dout == din { return None; }
            let (mut r, mut c) = pos;
            match dout {
                Dir::North => { r = if r == 0 { return None; } else { r - 1 }; },
                Dir::East => { c += 1; },
                Dir::South => { r += 1; },
                Dir::West => { c = if c == 0 { return None; } else { c - 1 }; }
            };
            if r == rows || c == cols { return None; }
            if cost + *board.get(r, c).unwrap() >= limit { return None; }
            if path.contains(&(r, c)) { return None; }
            Some(((r, c), dout, *board.get(r, c).unwrap()))
        })
        .collect::<Vec<_>>();
    variants.sort_by_key(|v| (distance(&v.0, &(rows - 1, cols - 1)), v.2));
    let mut limit = limit;
    variants.iter()
        .filter_map(|&(pos, dout, _)| {
            let rest = if dout == din { rest - 1 } else { 2 };
            let result = mincost(&board, &path, pos, rest, dout, cost, limit, depth - 1, step);
            if let Some(result) = result {
                limit = limit.min(result);
            }
            result
        })
        .min()        
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
        (vec![(0, 0), (0, 1)], 2, Dir::East, *board.get(0, 1).unwrap()),
        (vec![(0, 0), (1, 0)], 2, Dir::South, *board.get(1, 0).unwrap())
    ] {
        let Err(i) = paths.binary_search_by_key(&weight(&board, &path), |p| { weight(&board, p) }) else {
            panic!("Path duplicate");
        };
        paths.insert(i, path);
    }
    let mut step = 0;
    let result = mincost(&board, &vec![(0, 0)], (0, 1), 1, Dir::East, 0, usize::MAX, usize::MAX, &mut step)
        .min(mincost(&board, &vec![(0, 0)], (1, 0), 1, Dir::South, 0, usize::MAX, usize::MAX, &mut step));
    println!("{:?}", result);
}
