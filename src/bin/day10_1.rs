use std::{fs, env, io::{self, BufRead}};
use std::ops::Add;

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
struct P(isize, isize);

impl P {
    fn r(self: &P) -> isize { self.0 }
    fn c(self: &P) -> isize { self.1 }
}
impl Add for P {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines().map(|l| l.unwrap());
    let mut grid = grid::Grid::<Vec<P>>::new(0, 0);
    let mut symb = grid::Grid::<char>::new(0, 0);
    let mut s = None;
    for (i, line) in lines.enumerate() {
        let i = i as isize;
        let mut row = Vec::<Vec<P>>::new();
        let mut symb_row = Vec::<char>::new();
        for (j, c) in line.chars().enumerate() {
            let j = j as isize;
            row.push(
                match c {
                    '.' | 'S' => vec![],
                    '|' => vec![P(i-1, j), P(i+1, j)],
                    '-' => vec![P(i, j-1), P(i, j+1)],
                    'L' => vec![P(i-1, j), P(i, j+1)],
                    'J' => vec![P(i-1, j), P(i, j-1)],
                    '7' => vec![P(i, j-1), P(i+1, j)],
                    'F' => vec![P(i, j+1), P(i+1, j)],
                    c => { panic!("Invalid item '{}'", c); }
                }
            );
            if c == 'S' {
                s = Some(P(i, j));
                symb_row.push('7');
            } else {
                symb_row.push(c);
            }
        }
        grid.push_row(row);
        symb.push_row(symb_row);
    };
    let s = s.unwrap();
    let neighbours = vec![P(-1, 0), P(0, 1), P(1, 0), P(0, -1)].into_iter()
        .filter(|d| {
            let p = s + *d;
            0 <= p.r() && p.r() < (grid.rows() as isize)
                && 0 <= p.c() && p.c() < (grid.cols() as isize)
                && grid.get(p.r(), p.c()).unwrap().contains(&s)
        })
        .collect::<Vec<_>>();
    *symb.get_mut(s.r(), s.c()).unwrap() = match (neighbours[0], neighbours[1]) {
        (P(-1, 0), P(0, 1)) => 'L',
        (P(-1, 0), P(1, 0)) => '|',
        (P(-1, 0), P(0, -1)) => 'J',
        (P(0, 1), P(1, 0)) => 'F',
        (P(0, 1), P(0, -1)) => '-',
        (P(1, 0), P(0, -1)) => '7',
        b => { panic!("Invalid bend {:?}", b); }
    };
    
    let mut result = 1;
    let mut p1 = s;
    let mut p2 = vec![p1+P(-1, 0), p1+P(0, 1), p1+P(1, 0), p1+P(0, -1)].into_iter()
        .find(|p| {
            0 <= p.r() && p.r() < (grid.rows() as isize)
                && 0 <= p.c() && p.c() < (grid.cols() as isize)
                && grid.get(p.r(), p.c()).unwrap().iter().any(|p| *p == s)
        })
        .unwrap();
    while p2 != s {
        let p0 = p1;
        p1 = p2;
        p2 = *grid.get(p2.r(), p2.c()).unwrap().iter()
            .find(|p| **p != p0)
            .unwrap();
        result += 1;
    }
    result /= 2;
    println!("{}", result);
}
