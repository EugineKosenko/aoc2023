use std::{fs, env, io::{self, BufRead}};
use grid::Grid;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::collections::HashMap;

type Board = Grid<usize>;
type Pos = (usize, usize);
fn distance(p1: &Pos, p2: &Pos) -> usize {
    ((p1.0 as isize - p2.0 as isize).abs()
        + (p1.1 as isize - p2.1 as isize).abs()) as usize
}
#[derive(EnumIter, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, Debug)]
enum Dir { North, East, South, West }
type Path = Grid<usize>;
#[derive(PartialEq, Hash)]
struct Key(Pos, Dir, usize);

type Limits = HashMap<Key, usize>;
fn mincost(
    board: &Board, path: &mut Path, pos: Pos,
    rest: usize, din: Dir,
    cost: usize, limit: usize, limits: &mut Limits,
    depth: usize, step: &mut usize
) -> Option<usize> {
    *step += 1;
    if *step % 10_000_000 == 0 { println!("{} {} {}", *step, limit, board.rows() * board.cols() - depth); }
    if depth == 0 {
        //println!("{} {:?}", cost, path);
        return Some(cost);
    }
    let (rows, cols) = (board.rows(), board.cols());
    let cost = cost + *board.get(pos.0, pos.1).unwrap();
    limits.insert(Key(pos, din, rest), cost);
    if pos == (rows - 1, cols - 1) {
        println!("{} {}", limit, cost);
        //println!("{:#?}", show(rows, cols, &path));
        return Some(cost);
    }
    *path.get_mut(pos.0, pos.1).unwrap() = rows * cols - depth;
    let mut variants = Dir::iter()
        .filter_map(|dout| {
            if rest == 0 && dout == din { return None; }
            let (mut r, mut c) = pos;
            match dout {
                Dir::North => { r = if r == 0 { return None; } else { r - 1 }; },
                Dir::East => { c = if c == cols - 1 { return None } else { c + 1 }; },
                Dir::South => { r = if r == rows - 1 { return None } else { r + 1 }; },
                Dir::West => { c = if c == 0 { return None; } else { c - 1 }; }
            };
            if cost + *board.get(r, c).unwrap() >= limit { return None; }
            let rest = if dout == din { rest - 1 } else { 2 };
            if cost + *board.get(r, c).unwrap() >= *limits.get(&Key((r, c), dout, rest)).unwrap_or(&usize::MAX) { return None; }
            if *path.get(r, c).unwrap() > 0 { return None; }
            Some(((r, c), dout, *board.get(r, c).unwrap()))
        })
        .collect::<Vec<_>>();
    variants.sort_by_key(|v| (distance(&v.0, &(rows - 1, cols - 1)), v.2));
    let mut limit = limit;
    let result = variants.iter()
        .filter_map(|&(pos, dout, _)| {
            let rest = if dout == din { rest - 1 } else { 2 };
            let result = mincost(&board, path, pos, rest, dout, cost, limit, limits, depth - 1, step);
            if let Some(result) = result { limit = result; }
            result
        })
        .min();
    *path.get_mut(pos.0, pos.1).unwrap() = 0;
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
            row.push(c.to_digit(10).unwrap() as usize);
        }
        board.push_row(row);
    }
    let (rows, cols) = (board.rows(), board.cols());
    println!("{:#?}", board);
    let result = 0;
    let mut limits = Limits::new();
    let mut path = Path::new(rows, cols);
    path.fill(0);
    *path.get_mut(0, 0).unwrap() = 1;
    let mut step = 0;
    let result1 = mincost(
        &board, &mut path, (0, 1),
        1, Dir::East,
        0, usize::MAX, &mut limits,
        rows * cols - 1, &mut step);
    println!("First: {:?}", result1);
    let limit = result1.unwrap_or(usize::MAX);
    
    let result2 = mincost(
        &board, &mut path, (1, 0),
        1, Dir::South,
        0, limit, &mut limits,
        rows * cols - 1, &mut step);
    println!("Result: {:?} {:?}", result1, result2);
    println!("{:?}", result);
}
