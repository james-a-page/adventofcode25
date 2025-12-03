use itertools::Itertools;
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
    // let file1 = &args[2];
    // if let Ok(lines) = read_lines(file1) {
    //     sol1(lines)
    // }
    let file2 = &args[2];
    if let Ok(lines) = read_lines(file2) {
        sol2(lines)
    }
}
// ===========================================================================

// Solutions
// ===========================================================================
fn sol1(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    for line in lines.map_while(Result::ok) {
        let ranges: Vec<&str> = line.split(',').collect();
        for range in ranges {
            let mut x = range.split('-');
            let a: i64 = x.next().unwrap().parse().unwrap();
            let b: i64 = x.next().unwrap().parse().unwrap();
            // println!("{} {}", a, b);

            for i in a..(b + 1) {
                let s = i.to_string();
                // Odd length nums can't have a perfect repeat
                if s.len() % 2 == 0 {
                    let (first, last) = s.split_at(i.to_string().len() / 2);
                    if first.eq_ignore_ascii_case(last) {
                        total += i;
                        // println!("{}", i);
                    }
                }
            }
        }
    }
    println!("{}", total);
}

fn sol2(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    for line in lines.map_while(Result::ok) {
        let ranges: Vec<&str> = line.split(',').collect();
        for range in ranges {
            let mut x = range.split('-');
            let a: i64 = x.next().unwrap().parse().unwrap();
            let b: i64 = x.next().unwrap().parse().unwrap();
            // println!("{} {}", a, b);

            for i in a..(b + 1) {
                let s = i.to_string();

                for r in 1..s.len() {
                    if s.len() % r == 0 {
                        let parts: Vec<&str> = s
                            .as_bytes()
                            .chunks(r)
                            .map(|chunk| std::str::from_utf8(chunk).unwrap())
                            .collect();
                        // println!("{:?}", parts);
                        if parts.iter().all_equal() {
                            total += i;
                            println!("{}", i);
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{}", total);
}
// ===========================================================================
