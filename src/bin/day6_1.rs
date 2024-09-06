use std::{fs, env, io::{self, BufRead}};



fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap());
    let line = lines.next().unwrap();
    let times = line.split_whitespace()
        .skip(1)
        .map(|t| t.parse::<usize>().unwrap());
    let line = lines.next().unwrap();
    let dists = line.split_whitespace()
        .skip(1)
        .map(|d| d.parse::<usize>().unwrap());
    let mut result = 1;
    for (time, dist) in times.zip(dists) {
        let b = -(time as f32);
        let c = dist as f32;
        let d = (b * b - 4.0 * c).sqrt();
        let x1 = (-b - d) / 2.0;
        let x2 = (-b + d) / 2.0;
        let x1 = (if x1 == x1.round() { x1.round() + 1.0} else { x1.ceil() }) as usize;
        let x2 = (if x2 == x2.round() { x2.round() - 1.0} else { x2.floor() }) as usize;
        result *= x2 - x1 + 1;
    }
    println!("{}", result);
}
