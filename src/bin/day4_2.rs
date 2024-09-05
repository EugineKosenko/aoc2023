use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Default, Debug)]
struct NumSet(BTreeSet<usize>);
impl Deref for NumSet {
    type Target = BTreeSet<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NumSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl FromStr for NumSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.to_owned();
        let mut result = Self::default();
        lazy_static::lazy_static! {
            static ref RE_ITEM: regex::Regex = regex::Regex::new(r"^(?P<item>\d+)( )*(?P<rest>.*)$").unwrap();
        }
        while !line.is_empty() {
            let cps = RE_ITEM.captures(&line).unwrap();
            let item = cps.name("item").unwrap().as_str().parse::<usize>().unwrap();
            result.insert(item);
            line = cps.name("rest").unwrap().as_str().to_owned();
        }
        Ok(result)
    }
}

fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Vec<_>>();
    let mut scores = vec![1; lines.len()];
    let mut i = 0;
    for line in lines {
        let line = line.unwrap();
        lazy_static::lazy_static! {
            static ref RE_CARD: regex::Regex = regex::Regex::new(r"^Card( )+(?P<id>\d+):( )+(?P<win>[\d ]+) \|( )+(?P<all>[\d ]+)$").unwrap();
        }
        let cps = RE_CARD.captures(&line).unwrap();
        // let id = cps.name("id").unwrap().as_str().parse::<usize>().unwrap();
        let win = cps.name("win").unwrap().as_str().parse::<NumSet>().unwrap();
        let all = cps.name("all").unwrap().as_str().parse::<NumSet>().unwrap();
        result += scores[i];
        let count = win.intersection(&all).count();
        for j in (i+1)..(scores.len().min(i+count+1)) {
            scores[j] += scores[i];
        }
        i += 1;
    }
    println!("{}", result);
}
