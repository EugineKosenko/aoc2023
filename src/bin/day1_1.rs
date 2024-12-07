use std::{fs, env, io::{self, BufRead}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Vec<_>>();
    let mut s = 0;
    for line in lines {
        let line = line.unwrap();
        let digits = line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>();
        s += format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<usize>().unwrap();
    }
    println!("{}", s);
}
