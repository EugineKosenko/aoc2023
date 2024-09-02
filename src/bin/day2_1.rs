use std::{fs, env, io::{self, BufRead}};
use std::str::FromStr;

#[derive(Default, Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize
}

impl Round {
    fn is_fit(&self, limit: &Self) -> bool {
        return self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue;
    }
}
impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.to_owned();
        let mut result = Self::default();
        lazy_static::lazy_static! {
            static ref RE_ITEM: regex::Regex = regex::Regex::new(r"^(?P<count>\d+) (?P<color>red|green|blue)(, )?(?P<rest>.*)$").unwrap();
        }
        while !line.is_empty() {
            let cps = RE_ITEM.captures(&line).unwrap();
            let count = cps.name("count").unwrap().as_str().parse::<usize>().unwrap();
            let color = cps.name("color").unwrap().as_str().to_owned();
            line = cps.name("rest").unwrap().as_str().to_owned();
            result = match color.as_str() {
                "red" => Round { red: count,..result },
                "green" => Round { green: count,..result },
                "blue" => Round { blue: count,..result },
                color => { return Err(format!("Unknown color {}", color)); }
            };
        }
        Ok(result)
    }
}

fn main() {
    let limit = Round {
        red: 12,
        green: 13,
        blue: 14
    };
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Vec<_>>();
    let mut s = 0;
    for line in lines {
        let mut line = line.unwrap();
        let mut game_is_proper = true;
        lazy_static::lazy_static! {
            static ref RE_GAME: regex::Regex = regex::Regex::new(r"^Game (?P<id>\d+): (?P<rest>.+)$").unwrap();
        }
        let cps = RE_GAME.captures(&line).unwrap();
        let game_id = cps.name("id").unwrap().as_str().parse::<usize>().unwrap();
        line = cps.name("rest").unwrap().as_str().to_owned();
        lazy_static::lazy_static! {
            static ref RE_ROUND: regex::Regex = regex::Regex::new(r"^(?P<round>[^;]+)(; )?(?P<rest>.*)$").unwrap();
        }
        while game_is_proper && !line.is_empty() {
            let cps = RE_ROUND.captures(&line).unwrap();
            let round = cps.name("round").unwrap().as_str().to_owned();
            line = cps.name("rest").unwrap().as_str().to_owned();
            let round = round.parse::<Round>().unwrap();
            println!("{:?} --- {}", &round, round.is_fit(&limit));
            game_is_proper = round.is_fit(&limit);
        }
        if game_is_proper { s = s + game_id }
    }
    println!("{}", s);
}
