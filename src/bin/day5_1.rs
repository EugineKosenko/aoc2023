use std::{fs, env, io::{self, BufRead}};
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(PartialEq, Debug)]
struct Range {
    dest: usize,
    source: usize,
    length: usize
}

impl Range {
    fn map(&self, s: usize) -> Option<usize> {
        if self.source <= s && s < self.source + self.length {
            Some((s - self.source) + self.dest)
        } else {
            None
        }
    }
}
impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref RE_RANGE: regex::Regex = regex::Regex::new(r"^(?P<dest>\d+) (?P<source>\d+) (?P<length>\d+)$").unwrap();
        }
        let cps = RE_RANGE.captures(s).unwrap();
        let dest = cps.name("dest").unwrap().as_str().parse::<usize>().unwrap();
        let source = cps.name("source").unwrap().as_str().parse::<usize>().unwrap();
        let length = cps.name("length").unwrap().as_str().parse::<usize>().unwrap();
        Ok(Self { dest, source, length })
    }
}
#[derive(Default, PartialEq, Debug)]
struct Map(Vec<Range>);

impl Map {
    fn read<I: Iterator<Item = String>>(lines: &mut std::iter::Peekable<I>) -> Self {
        let mut result = Self::default();
        lines.next();
        while lines.peek().is_some() && lines.peek().unwrap() != "" {
            result.push(lines.next().unwrap().parse::<Range>().unwrap());
        }
        if lines.peek().is_some() { lines.next(); }
        result
    }
    fn map(&self, s: usize) -> usize {
        for range in self.iter() {
            if let Some(d) = range.map(s) {
                return d;
            }
        }
        s
    }
}
impl Deref for Map {
    type Target = Vec<Range>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .peekable();
    let mut seeds = Vec::<usize>::new();
    let line = lines.next().unwrap().to_owned();
    lazy_static::lazy_static! {
        static ref RE_SEEDS: regex::Regex = regex::Regex::new(r"^seeds: (?P<rest>[\d ]+)$").unwrap();
    }
    let cps = RE_SEEDS.captures(&line).unwrap();
    let mut line = cps.name("rest").unwrap().as_str().to_string();
    lazy_static::lazy_static! {
        static ref RE_SEED: regex::Regex = regex::Regex::new(r"^(?P<id>\d+)( )?(?P<rest>.*)$").unwrap();
    }
    while !line.is_empty() {
        let cps = RE_SEED.captures(&line).unwrap();
        let id = cps.name("id").unwrap().as_str().parse::<usize>().unwrap();
        seeds.push(id);
        line = cps.name("rest").unwrap().as_str().to_string();
    }
    lines.next();
    let mut maps = Vec::<Map>::new();
    while lines.peek().is_some() {
        let map = Map::read(&mut lines);
        maps.push(map);
    }
    let mut loc = None;
    for seed in seeds {
        let mut value = seed;
        for map in &maps {
            value = map.map(value);
        }
        loc = match loc {
            None => Some(value),
            Some(loc) => Some(loc.min(value))
        };
    }
    let result = loc.unwrap();
    println!("{}", result);
}
