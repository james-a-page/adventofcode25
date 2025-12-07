use std::collections::{HashMap, HashSet, hash_set};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, usize};

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
    let mut indexes: HashSet<usize> = HashSet::new();
    let mut first = true;
    let total: usize = lines.fold(0, |acc, mut line| {
        let mut increase = 0;
        let mut new_indexes: HashSet<usize> = HashSet::new();
        if first {
            let _ = new_indexes.insert(line.as_mut().unwrap().find("S").unwrap());
            first = false;
        }
        indexes.iter().for_each(|i| {
            if line.as_mut().unwrap().chars().nth(*i).unwrap() == '^' {
                increase += 1;
                let _ = new_indexes.insert(*i - 1);
                let _ = new_indexes.insert(*i + 1);
            } else {
                let _ = new_indexes.insert(*i);
            }
        });
        indexes = new_indexes;
        return acc + increase;
    });
    println!("{}", total)
}

fn sol2(lines: io::Lines<io::BufReader<File>>) {
    let mut indexes: HashMap<usize, usize> = HashMap::new();
    let mut first = true;
    lines.for_each(|mut line| {
        let mut new_indexes: HashMap<usize, usize> = HashMap::new();
        // Find init beam location
        if first {
            new_indexes.insert(line.as_mut().unwrap().find("S").unwrap(), 1);
            first = false;
        }
        indexes.iter().for_each(|(i, count)| {
            if line.as_mut().unwrap().chars().nth(*i).unwrap() == '^' {
                // if has already been reached combine number of paths it could have gotten there with,
                // else duplicate number of possibilities
                if new_indexes.contains_key(&(*i - 1)) {
                    new_indexes.insert(*i - 1, new_indexes[&(*i - 1)] + *count);
                } else {
                    new_indexes.insert(*i - 1, *count);
                }
                if new_indexes.contains_key(&(*i + 1)) {
                    new_indexes.insert(*i + 1, new_indexes[&(*i + 1)] + *count);
                } else {
                    new_indexes.insert(*i + 1, *count);
                }
            } else {
                // No split, propogate the value to next row but account for converging paths with a split beam
                if new_indexes.contains_key(i) {
                    new_indexes.insert(*i, new_indexes[i] + *count);
                } else {
                    new_indexes.insert(*i, *count);
                }
            }
        });
        indexes = new_indexes;
    });
    println!("{}", indexes.iter().fold(0, |total, (_, c)| total + c));
}
// ===========================================================================
