use std::{fs, env, io::{self, BufRead}};
use grid::Grid;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

type Board = Grid<usize>;
type Point = (usize, usize);
#[derive(enum_iterator::Sequence, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Dir { North, East, South, West }
type Pos = (Point, Dir, usize);
type Scores = BTreeMap<Pos, usize>;
type Poss = BTreeSet<(usize, usize, Pos)>;

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
    let mut scores = Scores::new();
    let mut poss = Poss::new();
    poss.insert(((rows - 1) + (cols - 1), 0, ((0, 0), Dir::East, 3)));
    let mut limit = usize::MAX;
    while let Some((dist, score, pos)) = poss.pop_first() {
        if dist == 0 {
            limit = limit.min(score);
            println!("{}", limit);
            continue;
        }
        if score >= limit { continue; }
        if score < *scores.get(&pos).unwrap_or(&usize::MAX) {
            scores.insert(pos, score);
        } else {
            continue;
        }
        let (point, dir, count) = pos;
        for next_dir in enum_iterator::all::<Dir>() {
            if count == 0 && next_dir == dir { continue; }
            let point = match dir {
                Dir::North => if point.0 == 0 { continue } else { (point.0 - 1, point.1) },
                Dir::East => if point.1 == cols - 1 { continue } else { (point.0, point.1 + 1) },
                Dir::South => if point.0 == rows - 1 { continue } else { (point.0 + 1, point.1) },
                Dir::West => if point.1 == 0 { continue } else { (point.0, point.1 - 1) }
            };
            let count = if next_dir == dir { count - 1 } else { 3 };
            let dist = (rows - 1 - point.0) + (cols - 1 - point.1);
            let score = score + board.get(point.0, point.1).unwrap();
            poss.insert((dist, score, (point, next_dir, count)));
        }
    }
    let result = limit;
    println!("{:?}", result);
}
