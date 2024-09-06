use std::{fs, env, io::{self, BufRead}};
use std::collections::HashMap;




fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap());
    let prog = lines.next().unwrap();
    lines.next();
    let mut links = HashMap::<String, (String, String)>::new();
    for line in lines {
        lazy_static::lazy_static! {
            static ref RE_LINK: regex::Regex = regex::Regex::new(r"^(?P<from>[A-Z0-9]{3}) = \((?P<left>[A-Z0-9]{3}), (?P<right>[A-Z0-9]{3})\)$").unwrap();
        }
        let cps = RE_LINK.captures(&line).unwrap();
        let from = cps.name("from").unwrap().as_str().to_owned();
        let left = cps.name("left").unwrap().as_str().to_owned();
        let right = cps.name("right").unwrap().as_str().to_owned();
        links.insert(from, (left, right));
    }
    let mut result = 0;
    let mut node = "AAA".to_string();
    let mut idx = 0;
    while node.as_str() != "ZZZ" {
        result += 1;
        let (left, right) = links.get(&node).unwrap();
        node = match &prog[idx..idx+1] {
            "L" => left.to_owned(),
            "R" => right.to_owned(),
            c => panic!("Invalid command {}", c)
        };
        idx += 1;
        if idx == prog.len() { idx = 0; }
    }
    println!("{}", result);
}
