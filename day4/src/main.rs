use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    println!("{}", determine_score(&"data.in"));
    println!("{}", count_duplicates(&"data.in"));
}

fn determine_score(p: &dyn AsRef<Path>) -> u32 {
    let mut bf = {
        let f = OpenOptions::new().read(true).open(p).unwrap();
        BufReader::new(f)
    };

    let mut u = 0;
    let mut line = String::new();
    while let Ok(1..) = bf.read_line(&mut line) {
        let s = line.trim();

        let card = s.split([':', '|']).collect::<Vec<&str>>();
        let win = card[1].split_ascii_whitespace().map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let nums = card[2].split_ascii_whitespace().map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mut cv = 0;

        for w in &win {
            if nums.contains(w) {
                if cv == 0 {
                    cv = 1;
                } else {
                    cv *= 2;
                }
            }
        }
        println!("{win:?} | {nums:?} => {cv}");
        u += cv;

        line.clear();
    }
    u
}

fn count_duplicates(p: &dyn AsRef<Path>) -> u32 {
    let mut bf = {
        let f = OpenOptions::new().read(true).open(p).unwrap();
        BufReader::new(f)
    };
    let mut map = HashMap::<u32, u32>::new();

    let mut u = 0;
    let mut cn = 0;
    let mut line = String::new();
    while let Ok(1..) = bf.read_line(&mut line) {
        cn += 1;
        map.entry(cn).and_modify(|f| *f += 1).or_insert(1);
        let s = line.trim();

        let card = s.split([':', '|']).collect::<Vec<&str>>();
        let win = card[1].split_ascii_whitespace().map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let nums = card[2].split_ascii_whitespace().map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        for _ in 0..map[&cn] {
            let mut start = cn;
            for w in &win {
                if nums.contains(w) {
                    start += 1;
                    map.entry(start).and_modify(|f| *f += 1).or_insert(1);
                }
            }
        }

        line.clear();
    }
    for e in map {
        u += e.1;
    }
    u
}
