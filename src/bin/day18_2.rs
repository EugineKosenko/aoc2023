use std::{fs, env, io::{self, BufRead}};



fn main() {
    let result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines().map(|line| line.unwrap());
    let (mut row, mut col) = (0, 0);
    let mut circuit = vec![];
    lazy_static::lazy_static! {
        static ref RE_ITEMS: regex::Regex = regex::Regex::new(r"^[URDL] \d+ \(#(?P<dist>[a-f\d]{5})(?P<dir>[0-3])\)$").unwrap();
    }
    for line in lines {
        let cps = RE_ITEMS.captures(&line).unwrap();
        let dir = cps.name("dir").unwrap().as_str().parse::<usize>().unwrap();
        let dist = usize::from_str_radix(cps.name("dist").unwrap().as_str(), 16).unwrap();
        for _ in 0..dist {
            match dir {
                3 => { row -= 1; },
                0 => { col += 1; },
                1 => { row += 1; },
                2 => { col -= 1; },
                _ => panic!("Invalid dir")
            };
            circuit.push((row, col));
        }
    }
    let top = circuit.iter().map(|p| p.0).min().unwrap();
    let bottom = circuit.iter().map(|p| p.0).max().unwrap();
    let left = circuit.iter().map(|p| p.1).min().unwrap();
    let right = circuit.iter().map(|p| p.1).max().unwrap();
    println!("({} {}) ({} {})", top, left, bottom, right);
    eprintln!("{:?}", result);
}
