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
fn sol1(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    for bank in lines.map_while(Result::ok) {
        // Scan bank for highest number, once found scan subsequent to get highest following number
        let mut digit1 = 0;
        let mut digit2 = 0;
        let mut idx: usize = 0;
        for (i, c) in bank.chars().enumerate() {
            let c_int = c.to_digit(10).unwrap();
            if i < bank.len() - 1 {
                if c_int > digit1 {
                    digit1 = c_int;
                    idx = i;
                }
            } else {
                digit2 = c_int;
            }
        }

        for c in bank.chars().skip(idx) {
            let c_int = c.to_digit(10).unwrap();
            if c_int > digit2 {
                digit2 = c_int
            }
        }
        total += (10 * digit1) + digit2;
    }
    println!("{}", total);
}

fn sol2(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    for bank in lines.map_while(Result::ok) {
        let mut array: [u32; 12] = [0; 12];
        let mut idx: usize = 0;
        let mut tmp: usize = 0;
        for k in 0..12 {
            tmp = idx;
            for (i, c) in bank.chars().enumerate().skip(idx) {
                let c_int = c.to_digit(10).unwrap();
                if i < bank.len() - (11 - k) {
                    if c_int > array[k] {
                        array[k] = c_int;
                        tmp = i;
                    }
                } else {
                    idx = tmp;
                    break;
                }
                idx = tmp;
            }
        }
        total += array.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
    }
    println!("{}", total);
}
// ===========================================================================
