use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc::Iter;

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
    let file1 = &args[2];
    if let Ok(lines) = read_lines(file1) {
        sol1(lines)
    }
    let file2 = &args[2];
    if let Ok(lines) = read_lines(file2) {
        sol2(lines)
    }
}
// ===========================================================================

// Solutions
// ===========================================================================
fn in_interval(x: usize, (a, b): (usize, usize)) -> bool {
    return x >= a && x <= b;
}

fn merge_intervals(intervals: &[(usize, usize)], (a, b): (usize, usize)) -> Vec<(usize, usize)> {
    let mut na = a;
    let mut nb = b;

    for &(ia, ib) in intervals {
        if ib >= na && ia <= nb {
            na = na.min(ia);
            nb = nb.max(ib);
        }
    }

    let mut result = Vec::new();

    for &(ia, ib) in intervals {
        if ib < na || ia > nb {
            result.push((ia, ib));
        }
    }

    // Add merged interval
    result.push((na, nb));
    result.sort_by_key(|x| x.0);
    result
}

fn sol1(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    let mut intervals: Vec<(usize, usize)> = vec![];
    let mut ranges = true;
    for line in lines.map_while(Result::ok) {
        if ranges {
            // Continue til line break
            if line == "" {
                ranges = false;
                continue;
            }
            let split: Vec<&str> = line.split("-").collect();
            let a: usize = split[0].parse().unwrap();
            let b: usize = split[1].parse().unwrap();
            intervals = merge_intervals(&intervals, (a, b));
        } else {
            for int in intervals.iter() {
                if in_interval(line.parse().unwrap(), *int) {
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("{}", total)
}

fn sol2(lines: io::Lines<io::BufReader<File>>) {
    let mut intervals: Vec<(usize, usize)> = vec![];
    for line in lines.map_while(Result::ok) {
        // Continue til line break
        if line == "" {
            break;
        }
        let split: Vec<&str> = line.split("-").collect();
        let a: usize = split[0].parse().unwrap();
        let b: usize = split[1].parse().unwrap();
        intervals.sort();
        println!("{:?}", intervals);
        intervals = merge_intervals(&intervals, (a, b));
    }
    println!("{:?}", intervals);
    println!(
        "{}",
        intervals.iter().fold(0, |acc, (a, b)| acc + (b - a) + 1)
    );
}
// ===========================================================================
