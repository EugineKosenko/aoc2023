use std::{fs, env, io::{self, BufRead}};



fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap());
    let line = lines.next().unwrap();
    let time = line.split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>().unwrap();
    let line = lines.next().unwrap();
    let dist = line.split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>().unwrap();
    let b = rug::Float::with_val(64, -(time as f32));
    let c = rug::Float::with_val(64, dist);
    let d = (b.clone() * b.clone() - rug::Float::with_val(64, 4.0) * c.clone()).sqrt();
    let x1 = (-b.clone() - d.clone()) / rug::Float::with_val(64, 2.0);
    let x2 = (-b.clone() + d.clone()).to_f64() / rug::Float::with_val(64, 2.0);
    let x1 = x1.to_u32_saturating_round(rug::float::Round::Up).unwrap();
    let x2 = x2.to_u32_saturating_round(rug::float::Round::Down).unwrap();
    let result = x2 - x1 + 1;
    println!("{}", result);
}
