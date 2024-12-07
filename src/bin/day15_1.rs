use std::{fs, env, io::{self, BufRead}};

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines().map(|l| l.unwrap());
    let line = lines.next().unwrap();
    let snippets = line.split(',');
    let result = snippets.fold(0, |a, s| a + hash(s));
    println!("{}", result);
}
