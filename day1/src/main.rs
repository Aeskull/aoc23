use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

const NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let f = OpenOptions::new().read(true).open("data.in").unwrap();
    let mut bf = BufReader::new(f);

    let mut sum = 0;
    let mut s = String::new();
    while let Ok(1..) = bf.read_line(&mut s) {
        let mut first = '\0';
        let mut last = '\0';
        let mut name = String::new();

        for c in s.chars() {
            if let Some(_) = c.to_digit(10) {
                if first == '\0' {
                    first = c
                }
                last = c;
            } else {
                name.push(c);
                if let Some(n) = is_num(&name) {
                    if first == '\0' {
                        first = n
                    }
                    last = n;
                }
            }
        }
        let n = [first, last]
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .expect("first⌢last does not parse to u32 (should be impossible)");
        sum += n;
        s.clear();
    }
    println!("{sum}");
}

fn is_num(s: &str) -> Option<char> {
    let is = s.chars().rev().collect::<String>();
    for (i, name) in NAMES.iter().enumerate() {
        let iname = name.chars().rev().collect::<String>();

        if is.len() >= iname.len() && iname == &is[0..iname.len()] {
            let c = char::from_digit((i as u32) + 1, 10)
                .expect("Index is more than 9 (should be impossible)");
            return Some(c);
        }
    }
    None
}
