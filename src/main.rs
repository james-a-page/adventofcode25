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
    let file1 = &args[2];
    if let Ok(lines) = read_lines(file1) {
        sol1(lines)
    }
    // let file2 = &args[2];
    // if let Ok(lines) = read_lines(file2) {
    //     sol2(lines)
    // }
}
// ===========================================================================

// Solutions
// ===========================================================================
fn sol1(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    for bank in lines.map_while(Result::ok) {
        // Scan bank for highest number, once found scan subsequent to get highest following number
        //
    }
    println!("{}", total);
}

fn sol2(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    for line in lines.map_while(Result::ok) {}
    println!("{}", total);
}
// ===========================================================================
