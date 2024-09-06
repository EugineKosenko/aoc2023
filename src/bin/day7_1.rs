use std::{fs, env, io::{self, BufRead}};
use std::fmt;
use std::str;
use std::collections::BTreeSet;
use std::cmp;

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}", match self {
                Card::D2 => "2",
                Card::D3 => "3",
                Card::D4 => "4",
                Card::D5 => "5",
                Card::D6 => "6",
                Card::D7 => "7",
                Card::D8 => "8",
                Card::D9 => "9",
                Card::T => "T",
                Card::J => "J",
                Card::Q => "Q",
                Card::K => "K",
                Card::A => "A"
            }
        )
    }
}
impl str::FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::D2),
            "3" => Ok(Card::D3),
            "4" => Ok(Card::D4),
            "5" => Ok(Card::D5),
            "6" => Ok(Card::D6),
            "7" => Ok(Card::D7),
            "8" => Ok(Card::D8),
            "9" => Ok(Card::D9),
            "T" => Ok(Card::T),
            "J" => Ok(Card::J),
            "Q" => Ok(Card::Q),
            "K" => Ok(Card::K),
            "A" => Ok(Card::A),
            s => Err(format!("Invalid card {}", s))
        }
    }
}
fn cards(s: &str) -> Vec<Card> {
    s.chars()
        .map(|c| c.to_string().parse::<Card>().unwrap())
        .collect::<Vec<_>>()
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    High, OnePair, TwoPair, Three, Full, Four, Five
}
#[derive(PartialEq, Eq, PartialOrd, Debug)]
struct Hand {
    kind: Kind,
    cards: Vec<Card>
}

impl Hand {
    
}
impl str::FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = cards(s);
        let kind = kind(&cards);
        Ok(Self {
            kind: kind,
            cards: cards
        })
    }
}
impl cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let result = match self.kind.cmp(&other.kind) {
            cmp::Ordering::Equal => {
                self.cards.cmp(&other.cards)
            },
            ord => ord
        };
        result
    }
}
#[test]
fn test_hand() {
    assert_eq!("32T3K".parse::<Hand>().unwrap(), Hand { kind: Kind::OnePair, cards: cards("32T3K") });
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    D2, D3, D4, D5, D6, D7, D8, D9, T, J, Q, K, A
}
fn kind(cards: &Vec<Card>) -> Kind {
  let set = cards.iter().collect::<BTreeSet<&Card>>();
  let mut counts = set.iter()
      .map(|c| {
          cards.iter()
              .filter(|c1| c1 == c)
              .count()
      })
      .collect::<Vec<_>>();
  counts.sort();
  counts.reverse();
  let counts = counts.iter()
      .map(|c| { format!("{}", c) })
      .collect::<String>();
  match counts.as_str() {
      "5" => Kind::Five,
      "41" => Kind::Four,
      "32" => Kind::Full,
      "311" => Kind::Three,
      "221" => Kind::TwoPair,
      "2111" => Kind::OnePair,
      "11111" => Kind::High,
      v => { panic!("Invalid counts {:?}", v) }
  }
}
#[test]
fn test_kind_1() {
    assert_eq!(kind(&cards("32T3K")), Kind::OnePair);
    assert_eq!(kind(&cards("T55J5")), Kind::Three);
    assert_eq!(kind(&cards("KK677")), Kind::TwoPair);
    assert_eq!(kind(&cards("KTJJT")), Kind::TwoPair);
    assert_eq!(kind(&cards("QQQJA")), Kind::Three);
}
#[test]
fn test_hand_cmp() {
    let h1 = "KK677".parse::<Hand>().unwrap();
    let h2 = "KTJJT".parse::<Hand>().unwrap();
    assert!(h2 < h1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap());
    let mut bids = lines
        .map(|line| {
            let mut items = line.split_whitespace();
            let hand = items.next().unwrap().parse::<Hand>().unwrap();
            let bid = items.next().unwrap().parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();
    let mut result = 0;
    bids.sort_by(|b1, b2| b1.0.cmp(&b2.0));
    for i in 0..bids.len() {
        result += (i + 1) * bids[i].1;
    }
    println!("{}", result);
}
