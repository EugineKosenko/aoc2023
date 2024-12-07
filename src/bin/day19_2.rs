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
    fn size(&self, set: Set, program: &Program) -> usize {
        let mut result = 0;
        let mut set = set;
        for rule in &self.0 {
            let (size, rest) = rule.filter(set, program);
            result += size;
            match rest {
                None => { break; },
                Some(rest) => { set = rest; }
            }
        }
        result
    }
}
impl Rule {
    fn filter(&self, set: Set, program: &Program) -> (usize, Option<Set>) {
        let (set, rest) = match &self.cond {
            None => (Some(set), None),
            Some(cond) => set.split(cond)
        };
        let size = match self.flow.as_str() {
            "R" => 0,
            "A" => match set {
                None => 0,
                Some(set) => set.size()
            },
            flow => match set {
                None => 0,
                Some(set) => program.get(flow).unwrap().size(set, program)
            }
        };
        (size, rest)
    }
}
#[derive(Debug, Clone)]
struct Set(BTreeMap<String, Range>);

impl Set {
    fn size(&self) -> usize { self.0.values().map(|range| range.size()).product() }
    fn split(&self, cond: &Cond) -> (Option<Self>, Option<Self>) {
        let (selected, rest) = self.0.get(&cond.cat).unwrap().split(cond.sign, cond.value);
        (selected.map(|selected| {
            let mut set = self.0.clone();
            set.insert(cond.cat.clone(), selected);
            Self(set)
        }),
         rest.map(|rest| {
            let mut set = self.0.clone();
            set.insert(cond.cat.clone(), rest);
            Self(set)
        }))
    }
}
#[derive(Debug, Clone)]
struct Range(usize, usize);

impl Range {
    fn size(&self) -> usize { self.1 - self.0 + 1 }
    fn split(&self, sign: char, value: usize) -> (Option<Self>, Option<Self>) {
        match sign {
            '<' => {
                if value <= self.0 {
                    (None, Some(self.clone()))
                } else if self.1 < value {
                    (Some(self.clone()), None)
                } else {
                    (Some(Self(self.0, value - 1)), Some(Self(value, self.1)))
                }
            },
            '>' => {
                if value < self.0 {
                    (Some(self.clone()), None)
                } else if self.1 <= value {
                    (None, Some(self.clone()))
                } else {
                    (Some(Self(value + 1, self.1)), Some(Self(self.0, value)))
                }
            },
            sign => panic!("Unexpected sign {}", sign)
        }
    }
}

fn main() {
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
    let set = Set(BTreeMap::from([
        ("x".to_string(), Range(1, 4000)),
        ("m".to_string(), Range(1, 4000)),
        ("a".to_string(), Range(1, 4000)),
        ("s".to_string(), Range(1, 4000))
    ]));
    let result = program.get("in").unwrap().size(set, &program);
    eprintln!("{:?}", result);
}
