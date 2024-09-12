use std::{fs, env, io::{self, BufRead}};

fn proper(mode: char, lead: char, springs: &[char], si: usize, counts: &Vec<usize>, progress: &mut usize, solution: &String) -> usize {
    let mut mode = mode;
    let mut counts = counts.clone();
    let mut solution = solution.clone();
    let mut lead = lead;
    let mut si = si;
    loop {
        if lead == '?' {
            let mut s1 = solution.clone();
            let r1 = proper(mode, '.', &springs, si, &counts, progress, &s1);
            let mut s2 = solution.clone();
            let r2 = proper(mode, '#', &springs, si, &counts, progress, &s2);
            let result = r1 + r2;
            return result;
        }
        solution.push(lead);
        match (mode, lead) {
            ('.', '.') => {
                /* nothing */
            },
            ('.', '#') => {
                if counts.is_empty() || counts[0] == 0 { return 0; }
                counts[0] -= 1;
                mode = '#';
            },
            ('#', '.') => {
                if counts.is_empty() || counts[0] > 0 { return 0; }
                counts.remove(0);
                mode = '.';
            },
            ('#', '#') => {
                if counts.is_empty() || counts[0] == 0 { return 0; }
                counts[0] -= 1;
            },
            c => { panic!("Invalid combination {:?}", c); }
        }
        if si == springs.len() { break; }
        lead = springs[si];
        si += 1;
    }
    if mode == '.' {
        if counts.is_empty() {
            *progress += 1;
            println!("solution: {}", solution);
            if *progress % 1000000 == 0 { println!("progress: {}", progress); }
            1
        } else {
            0
        }
    } else {
        if counts.len() == 1 && counts[0] == 0 {
            *progress += 1;
            println!("solution: {}", solution);
            if *progress % 1000000 == 0 { println!("progress: {}", progress); }
            1
        } else {
            0
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines().map(|l| l.unwrap());
    let result: usize = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let springs = parts.next().unwrap()
                .chars()
                .collect::<Vec<_>>();
            let counts = parts.next().unwrap();
            let counts = counts
                .split(',')
                .map(|count| count.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut progress = 0;
            let solution = String::new();
            let result = proper('.', springs[0], &springs, 1, &counts, &mut progress, &solution);
            println!("{}", result);
            result
        })
        .sum();
    println!("{}", result);
}
