use std::{fs, env, io::{self, BufRead}};

fn reduce(src: &[isize]) -> Vec<isize> {
    let mut result = Vec::new();
    for i in 0..src.len()-1 {
        result.push(src[i+1]-src[i]);
    }
    result
}
#[test]
fn test_reduce() {
    assert_eq!(reduce(&vec![10, 13, 16, 21, 30, 45]), vec![3, 3, 5, 9, 15]);
    assert_eq!(reduce(&vec![3, 3, 5, 9, 15]), vec![0, 2, 4, 6]);
    assert_eq!(reduce(&vec![0, 2, 4, 6]), vec![2, 2, 2]);
    assert_eq!(reduce(&vec![2, 2, 2]), vec![0, 0]);
}
fn extend(src: &[isize]) -> isize {
    if src.iter().all(|v| *v == 0) {
        0
    } else {
        src.last().unwrap() + extend(&reduce(src))
    }
}
#[test]
fn test_extend() {
    assert_eq!(extend(&vec![0, 0]), 0);
    assert_eq!(extend(&vec![2, 2, 2]), 2);
    assert_eq!(extend(&vec![0, 2, 4, 6]), 8);
    assert_eq!(extend(&vec![3, 3, 5, 9, 15]), 23);
    assert_eq!(extend(&vec![10, 13, 16, 21, 30, 45]), 68);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let seqs = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|v| v.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        });
    let mut result = 0;
    for seq in seqs {
        result += extend(&seq);
    }
    println!("{}", result);
}
