use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn main() {
    let f = OpenOptions::new().read(true).open("data.in").unwrap();
    let mut bf = BufReader::new(f);

    let mut line = String::new();
    let mut symbols = vec![];
    let mut digits = vec![];
    let mut line_count = 0;
    while let Ok(1..) = bf.read_line(&mut line) {
        let s = line.trim();
        let mut num = String::new();
        let mut start = None;

        for (l, c) in s.char_indices() {
            match c {
                '0'..='9' => {
                    start.get_or_insert(l);
                    num.push(c);
                }
                _ => {
                    if let Some(start) = start.take() {
                        let range = start..l;
                        let Ok(n) = num.parse::<u32>() else {
                            return;
                        };
                        num.clear();
                        digits.push((line_count, range, n));
                    }
                    if c != '.' {
                        symbols.push((line_count, l, c));
                    }
                }
            }
        }

        if let Some(start) = start {
            let range = start..s.len();
            let Ok(n) = num.parse::<u32>() else {
                return;
            };
            num.clear();
            digits.push((line_count, range, n));
        }

        line.clear();
        line_count += 1;
    }

    println!("{:?}", parse_adjacencies(&digits, &symbols));
}

fn parse_adjacencies(
    digits: &[(usize, Range<usize>, u32)],
    symbols: &[(usize, usize, char)],
) -> (u32, u32) {
    let mut res = 0;
    let mut gr = 0;
    for (y, x, c) in symbols {
        let mut gvs = vec![];
        'dig: for (line, range, num) in digits {
            for nx in range.clone() {
                if nx.abs_diff(*x) <= 1 && line.abs_diff(*y) <= 1 {
                    res += num;
                    if *c == '*' {
                        gvs.push(num);
                    }
                    continue 'dig;
                }
            }
        }
        if gvs.len() == 2 {
            gr += gvs.iter().fold(1, |acc, x| acc * **x);
        }
    }
    (res, gr)
}
