use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;
use all_asserts::assert_lt;

type Chunk = Vec<char>;
#[contracts::debug_requires(ispring <= chunk.len())]
#[contracts::debug_ensures(ret.iter().all(|i| *i <= chunk.len()))]
fn fixed(chunk: &Chunk, ispring: usize, count: usize) -> Vec<usize> {
    if count > chunk.len() - ispring {
        return Vec::with_capacity(200);
    }
    if chunk[ispring] == '#' {
        let mut result = Vec::with_capacity(200);
        if ispring + count == chunk.len() {
            result.push(ispring + count);
        } else if chunk[ispring + count] == '?' {
            result.push(ispring + count + 1);
        }
        // memo.insert((chunk.to_vec(), ispring, count), result.clone());
        return result;
    }     
    let mut result = fixed(chunk, ispring + 1, count);
    if ispring + count == chunk.len() {
        result.push(ispring + count);
    } else if chunk[ispring + count] == '?' {
        result.push(ispring + count + 1);
    }
    // memo.insert((chunk.to_vec(), ispring, count), result.clone());
    return result;
}
#[test]
fn test_fixed() {
    assert_eq!(fixed(&vec!['#'], 0, 1), vec![1]);
    assert_eq!(fixed(&vec!['?', '?', '?', '?', '?'], 2, 2), vec![5, 5]);
    assert_eq!(fixed(&vec!['?', '?', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?', '?'], 0, 2), vec![5]);
    assert_eq!(fixed(&vec!['?', '?', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?', '?'], 3, 2), vec![6]);
    assert_eq!(fixed(&vec!['?', '?', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?', '?'], 1, 2), vec![5]);
    assert_eq!(fixed(&vec!['?', '?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'], 1, 2), vec![]);
    assert_eq!(fixed(&vec!['?', '?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'], 0, 6), vec![9, 8, 7]);
}
#[contracts::debug_requires(ispring <= chunk.len())]
fn is_total(chunk: &Chunk, ispring: usize) -> bool {
    chunk.iter()
        .skip(ispring)
        .all(|s| *s == '?')
}
#[test]
fn test_is_total() {
    assert!(is_total(&vec![], 0));
    assert!(!is_total(&vec!['#'], 0));
    assert!(is_total(&vec!['?', '?', '?', '?', '?'], 0));
    assert!(!is_total(&vec!['?', '?', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?', '?'], 0));
    assert!(is_total(&vec!['?', '?', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?', '?'], 4));
}
#[contracts::debug_requires(ichunk <= chunks.len())]
#[contracts::debug_requires(ichunk == chunks.len() -> ispring == 0)]
#[contracts::debug_requires(ichunk < chunks.len() -> ispring <= chunks[ichunk].len())]
fn is_possible(chunks: &Vec<Chunk>, ichunk: usize, ispring: usize, counts_len: usize) -> bool {
    counts_len >=
        if ichunk == chunks.len() {
            0
        } else {
            (if is_total(&chunks[ichunk], ispring) { 0 } else { 1 })
            + chunks.iter()
                .skip(ichunk + 1)
                .filter(|c| !is_total(c, 0))
                .count()
        }
}
#[test]
fn test_is_possible() {
    assert!(is_possible(&vec![
        vec!['#'], vec!['#'], vec!['#', '#', '#'],
        vec!['?'], vec!['#'], vec!['#'], vec!['#', '#', '#']
    ], 0, 0, 6));
    assert!(!is_possible(&vec![
        vec!['#'], vec!['#'], vec!['#', '#', '#'],
        vec!['#'], vec!['#'], vec!['#'], vec!['#', '#', '#']
    ], 0, 0, 6));
    assert!(is_possible(&vec![
        vec!['#'], vec!['#'], vec!['#', '#', '#'],
        vec!['#'], vec!['#'], vec!['#'], vec!['#', '#', '#']
    ], 0, 1, 6));
    assert!(is_possible(&vec![
        vec!['#'], vec!['#'], vec!['#', '#', '#'],
        vec!['#'], vec!['#'], vec!['#'], vec!['#', '#', '#']
    ], 1, 0, 6));
    assert!(is_possible(&vec![
        vec!['#'], vec!['#'], vec!['#', '#', '#'],
        vec!['#'], vec!['#'], vec!['#'], vec!['#', '#', '#']
    ], 1, 1, 5));
}
type Memo = BTreeMap<(usize, usize, usize), usize>;

#[contracts::debug_requires(ichunk <= chunks.len())]
#[contracts::debug_requires(ichunk == chunks.len() -> ispring == 0)]
#[contracts::debug_requires(ichunk < chunks.len() -> ispring <= chunks[ichunk].len())]
#[contracts::debug_requires(icount <= counts.len())]
fn solutions(memo: &mut Memo, chunks: &Vec<Chunk>, ichunk: usize, ispring: usize, counts: &Vec<usize>, icount: usize) -> usize {
    if let Some(result) = memo.get(&(ichunk, ispring, icount)) {
        return *result;
    }
    let chunks_is_empty = ichunk == chunks.len();
    let counts_is_empty = icount == counts.len();
    if chunks_is_empty && counts_is_empty {
        memo.insert((ichunk, ispring, icount), 1);
        return 1;
    }
    if chunks_is_empty && !counts_is_empty {
        memo.insert((ichunk, ispring, icount), 0);
        return 0;
    }
    if !chunks_is_empty &&
        counts_is_empty {
            let result = if is_total(&chunks[ichunk], ispring)
                && chunks.iter().skip(ichunk + 1).all(|c| is_total(c, 0)) { 1 } else { 0 };
            memo.insert((ichunk, ispring, icount), result);
            return result;
        }
    all_asserts::debug_assert_lt!(ichunk, chunks.len());
    all_asserts::debug_assert_lt!(icount, counts.len());
    if !is_possible(chunks, ichunk, ispring, counts.len() -  icount) {
        memo.insert((ichunk, ispring, icount), 0);
        return 0;
    }
    let result = fixed(&chunks[ichunk], ispring, counts[icount]).iter()
        .map(|irest| {
            if *irest == chunks[ichunk].len() {
                solutions(memo, chunks, ichunk + 1, 0, counts, icount + 1)
            } else {
                solutions(memo, chunks, ichunk, *irest, counts, icount + 1)
            }
        })
        .sum::<usize>()
        + (if is_total(&chunks[ichunk], ispring) { solutions(memo, chunks, ichunk + 1, 0, counts, icount) } else { 0 });
    memo.insert((ichunk, ispring, icount), result);
    result
}
#[test]
fn test_solutions() {
    let mut memo = Memo::new();
    assert_eq!(solutions(&mut memo, &vec![
        vec!['?', '?', '?']
    ], 0, 0, &vec![1, 1], 0), 1);

    let mut memo = Memo::new();
    assert_eq!(solutions(&mut memo, &vec![
        vec!['?', '?', '?'], vec!['#', '#', '#']
    ], 0, 0, &vec![1, 1, 3], 0), 1);

    let mut memo = Memo::new();
    assert_eq!(solutions(&mut memo, &vec![vec!['#']], 0, 0, &vec![1], 0), 1);
    let mut memo = Memo::new();
    assert_eq!(solutions(&mut memo, &vec![
        vec!['#'], vec!['#'], vec!['#', '#', '#']
    ], 0, 0, &vec![1, 1, 3], 0), 1);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines().map(|l| l.unwrap());
    let result: usize = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let springs = parts.next().unwrap().to_string();
            let counts = parts.next().unwrap()
                .split(',')
                .map(|count| count.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mult = args[2].parse::<usize>().unwrap();
            let mut copy = springs.clone();
            for _ in 0..(mult-1) {
                copy = copy + "?";
                copy += &springs;
            }
            let springs = copy;
            let mut copy = counts.clone();
            for _ in 0..(mult-1) {
                copy.append(&mut counts.clone());
            }
            let counts = copy;
            let chunks = springs
                .split('.')
                .filter(|c| *c != "")
                .map(|c| c.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let mut memo = Memo::new();
            let result = solutions(&mut memo, &chunks, 0, 0, &counts, 0);
            result
        })
        .sum();
    println!("{}", result);
}
