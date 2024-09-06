use std::{fs, env, io::{self, BufRead}};
use std::collections::HashMap;


fn gcd(a: usize, b: usize) -> usize{
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

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
    let mut result: usize = 1;
    let nodes = links.keys()
        .filter(|n| &n[n.len()-1..n.len()] == "A");
    for node in nodes {
        let mut node = node;
        let mut cycle = 0;
        let mut idx = 0;
        while &node[node.len()-1..node.len()] != "Z" {
            cycle += 1;
            let (left, right) = links.get(node.as_str()).unwrap();
            node = match &prog[idx..idx+1] {
                "L" => left,
                "R" => right,
                c => panic!("Invalid command {}", c)
            };
            idx += 1;
            if idx == prog.len() { idx = 0; }
        }
        result = lcm(result, cycle);
    }
    println!("{}", result);
}
