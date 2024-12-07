use std::collections::BTreeMap;
use std::str;
use std::{fs, env, io::{self, BufRead}};

type Program = BTreeMap<String, Flow>;
#[derive(Debug)]
struct Flow(Vec<Rule>);

impl Flow {
    
}
impl str::FromStr for Flow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Flow(
            s.split(',')
                .map(|rule| rule.parse().unwrap())
                .collect()))
    }       
}
#[derive(Debug)]
struct Rule {
    cond: Option<Cond>,
    flow: String
}

impl Rule {
    
}
impl str::FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref RE_RULE: regex::Regex = regex::Regex::new(r"^(?P<cat>[xmas])(?P<sign>[<>])(?P<value>\d+):(?P<flow>[a-zAR]+)$").unwrap();
        }
        let (cond, flow) = match RE_RULE.captures(s) {
            None => { (None, s.to_string()) },
            Some(cps) => {
                let cat = cps.name("cat").unwrap().as_str().to_string();
                let sign = cps.name("sign").unwrap().as_str().chars().next().unwrap();
                let value = cps.name("value").unwrap().as_str().parse::<usize>().unwrap();
                let flow = cps.name("flow").unwrap().as_str().to_string();
                (Some(Cond { cat, sign, value }), flow)
            }
        };
        Ok(Rule { cond, flow })
    }       
}
#[derive(Debug)]
struct Cond {
    cat: String,
    sign: char,
    value: usize
}
impl Flow {
    fn next(&self, part: &Part) -> String {
        for rule in &self.0 {
            if let Some(flow) = rule.switch(part) {
                return flow;
            }
        }
        unreachable!();
    }
}
impl Rule {
    fn switch(&self, part: &Part) -> Option<String> {
        match &self.cond {
            None => Some(self.flow.clone()),
            Some(cond) => {
                if match cond.sign {
                    '<' => *part.get(&cond.cat).unwrap() < cond.value,
                    '>' => *part.get(&cond.cat).unwrap() > cond.value,
                    sign => panic!("Unexpected sign {}", sign)
                } { Some(self.flow.clone()) } else { None }
            }
        }
    }
}
type Part = BTreeMap<String, usize>;

fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines().map(|line| line.unwrap());
    let mut program = Program::new();
    for line in lines.by_ref() {
        if line.is_empty() { break; }
        lazy_static::lazy_static! {
            static ref RE_FLOW: regex::Regex = regex::Regex::new(r"^(?P<name>[a-z]+)\{(?P<flow>.+)\}$").unwrap();
        }
        let cps = RE_FLOW.captures(&line).unwrap();
        let name = cps.name("name").unwrap().as_str().to_string();
        let flow = cps.name("flow").unwrap().as_str().parse::<Flow>().unwrap();
        program.insert(name, flow);
    }
    for line in lines {
        let mut part = Part::new();
        for bind in line
            .strip_prefix('{').unwrap()
            .strip_suffix('}').unwrap()
            .split(',') {
                let (cat, value) = bind.split_once('=').unwrap();
                part.insert(cat.to_string(), value.parse().unwrap());
            }
        let mut flow = "in".to_string();
        while !["A", "R"].contains(&flow.as_str()) {
            flow = program.get(&flow).unwrap().next(&part);
        }
        if flow == "A" {
            result += part.values().sum::<usize>();
        }
    }
    eprintln!("{:?}", result);
}
