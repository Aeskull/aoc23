use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

const VALS: [i32; 3] = [12, 13, 14];

fn main() {
    let f = OpenOptions::new().read(true).open("data.in").unwrap();
    let mut bf = BufReader::new(f);

    let mut game_sum = 0;
    let mut power_sum = 0;
    let mut s = String::new();
    while let Ok(1..) = bf.read_line(&mut s) {
        let tokens = s.split([':', ';', '\r', '\n']).collect::<Vec<&str>>();
        let games = tokens
            .iter()
            .map(|f| f.split([' ', ',']).collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let gn = games[0][1].parse::<u32>().unwrap();

        if game_is_valid(&games[1..]) {
            game_sum += gn;
        }

        power_sum += power_set(&games[1..]);

        s.clear();
    }
    println!("Games: {game_sum}");
    println!("Power: {power_sum}");
}

fn game_is_valid(s: &[Vec<&str>]) -> bool {
    for round in s {
        let mut r = VALS[0];
        let mut g = VALS[1];
        let mut b = VALS[2];
        for color in round.chunks_exact(3) {
            let c = color[2];
            let n = color[1].parse::<i32>().unwrap();

            match c {
                "red" => {
                    r -= n;
                    if r < 0 {
                        return false;
                    }
                }
                "green" => {
                    g -= n;
                    if g < 0 {
                        return false;
                    }
                }
                "blue" => {
                    b -= n;
                    if b < 0 {
                        return false;
                    }
                }
                _ => return true,
            }
        }
    }

    true
}

fn power_set(s: &[Vec<&str>]) -> u32 {
    let mut maxes = [0u32; 3];
    for round in s {
        for color in round.chunks_exact(3) {
            let c = color[2];
            let n = color[1].parse::<u32>().unwrap();

            match c {
                "red" if maxes[0] < n => maxes[0] = n,
                "green" if maxes[1] < n => maxes[1] = n,
                "blue" if maxes[2] < n => maxes[2] = n,
                _ => {}
            }
        }
    }
    let power = maxes.iter().fold(1, |acc, x| x * acc);
    power
}
