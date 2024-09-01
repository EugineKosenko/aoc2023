use std::{fs, env, io::{self, BufRead}};




fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Vec<_>>();
    lazy_static::lazy_static! {
        static ref RE_LINE: regex::Regex = regex::Regex::new(r"^(?P<digit>\d|one|two|three|four|five|six|seven|eight|nine)(?P<rest>.*)$").unwrap();
    }
    let mut s = 0;
    for line in lines {
        let mut line = line.unwrap();
        let mut first = None;
        let mut last = None;
        while !line.is_empty() {
            if let Some(cps) = RE_LINE.captures(&line) {
                let digit = match cps.name("digit").unwrap().as_str() {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    d => d.parse::<usize>().unwrap()
                };
                if first.is_none() { first = Some(digit); }
                last = Some(digit);
            }
            line.remove(0);
        }
        s += format!("{}{}", first.unwrap(), last.unwrap())
            .parse::<usize>().unwrap();
    }
    println!("{}", s);
}
