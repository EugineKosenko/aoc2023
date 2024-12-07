use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;

fn hash(s: &str) -> usize {
    s.chars().fold(0, |a, c| ((a + c as usize) * 17) % 256)
}
#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("rn"), 0);
    assert_eq!(hash("cm"), 0);
    assert_eq!(hash("ot"), 3);
}
type Map = BTreeMap<usize, Vec<(String, usize)>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines().map(|l| l.unwrap());
    let line = lines.next().unwrap();
    let snippets = line.split(',');
    let mut result = 0;
    let mut map = Map::new();
    for snippet in snippets {
        if snippet.contains('=') {
            let mut lens = snippet.split('=');
            let label = lens.next().unwrap();
            let length = lens.next().unwrap().parse::<usize>().unwrap();
            let n = hash(label);
            map.entry(n).or_default();
            let box_ = map.get_mut(&n).unwrap();
            let mut lens_not_found = true;
            for item in box_.iter_mut() {
                if item.0 == label {
                    item.1 = length;
                    lens_not_found = false;
                    break;
                }
            }
            if lens_not_found { box_.push((label.to_owned(), length)); }
        } else {
            let label = &snippet[..snippet.len()-1];
            let n = hash(label);
            map.entry(n).or_default();
            let box_ = map.get(&n).unwrap();
            map.insert(
                n, box_.iter()
                    .filter_map(|(lb, ln)| {
                        if lb != label { Some((lb.to_owned(), *ln)) } else { None }
                    })
                    .collect::<Vec<_>>());
        }
    }
    for (n, box_) in map {
        for (i, item) in box_.iter().enumerate() {
            result += (n+1) * (i+1) * item.1;
        }
    }
    println!("{}", result);
}
