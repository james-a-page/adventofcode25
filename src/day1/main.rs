use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Default IO
// ===========================================================================
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file1 = &args[2];
    if let Ok(lines) = read_lines(file1) {
        sol1(lines)
    }
    let file2 = &args[3];
    if let Ok(lines) = read_lines(file2) {
        sol2(lines)
    }
}
// ===========================================================================

// Solutions
// ===========================================================================
fn apply(direction: char, amount: i32, origin: i32) -> i32 {
    match direction {
        'L' => (origin - amount).rem_euclid(100),
        'R' => (origin + amount).rem_euclid(100),
        _ => origin,
    }
}

fn sol1(lines: io::Lines<io::BufReader<File>>) {
    let mut pos: i32 = 50;
    let mut pw = 0;
    for line in lines.map_while(Result::ok) {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let amount: i32 = chars.collect::<String>().parse().unwrap();
        pos = apply(dir, amount, pos);
        if pos == 0 {
            pw += 1
        }
    }
    println!("{}", pw)
}

fn sol2(lines: io::Lines<io::BufReader<File>>) {
    let mut pos: i32 = 50;
    let mut pw = 0;
    for line in lines.map_while(Result::ok) {
        // println!("{}", pos);
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let amount: i32 = chars.collect::<String>().parse().unwrap();
        for _ in 0..amount {
            pos = match dir {
                'L' => {
                    if pos == 0 {
                        99
                    } else {
                        pos - 1
                    }
                }
                'R' => {
                    if pos == 99 {
                        0
                    } else {
                        pos + 1
                    }
                }
                _ => unreachable!(),
            };

            if pos == 0 {
                pw += 1;
            }
        }
    }
    println!("{}", pw);
}
// ===========================================================================
