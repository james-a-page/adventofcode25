use std::collections::{HashMap, HashSet, hash_set};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{array, env, i64};

use itertools::Itertools;

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

fn dist(a: Vec<i64>, b: Vec<i64>) -> f64 {
    let mut total = 0;
    for (i, ai) in a.iter().enumerate() {
        total += (ai - b[i]) * (ai - b[i]);
    }
    (total as f64).sqrt()
}

fn precompute_distances(positions: &Vec<Vec<i64>>) -> HashMap<(usize, usize), f64> {
    let mut distance: HashMap<(usize, usize), f64> = HashMap::new();
    let positions2 = positions.clone();
    for (i, pos) in positions.into_iter().enumerate() {
        for (j, pos2) in positions2.clone().into_iter().enumerate() {
            if i != j && !distance.contains_key(&(j, i)) {
                distance.insert((i, j), dist(pos.clone(), pos2));
            }
        }
    }
    return distance;
}
// Solutions
// ===========================================================================
fn sol1(lines: io::Lines<io::BufReader<File>>) {
    let positions: Vec<Vec<i64>> = lines
        .map(|x| {
            x.unwrap()
                .split(",")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let dist_lookup: HashMap<(usize, usize), f64> = precompute_distances(&positions);

    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut conn_count = 0;
    for ((i, j), _) in dist_lookup
        .clone()
        .iter()
        .sorted_by(|(_, v1), (_, v2)| v1.total_cmp(v2))
    {
        let mut idx_to_extend = 0;
        let mut idx_to_delete = 0;
        let mut extend_flag = false;
        let mut delete_flag = false;
        for (idx, c) in circuits.iter().enumerate() {
            if c.contains(i) || c.contains(j) {
                if extend_flag {
                    delete_flag = true;
                    idx_to_delete = idx;
                } else {
                    extend_flag = true;
                    idx_to_extend = idx;
                }
            }
        }

        if delete_flag {
            let new_circuit: HashSet<usize> = circuits
                .get(idx_to_extend)
                .unwrap()
                .union(circuits.get(idx_to_delete).unwrap())
                .map(|x| *x)
                .collect();
            circuits.remove(idx_to_delete);
            circuits.remove(idx_to_extend);
            circuits.push(new_circuit);
        } else if extend_flag {
            let mut new_circuit = circuits.get(idx_to_extend).unwrap().clone();
            new_circuit.insert(*i);
            new_circuit.insert(*j);
            circuits.remove(idx_to_extend);
            circuits.push(new_circuit);
        } else {
            circuits.push(HashSet::from([*i, *j]))
        }

        conn_count += 1;
        // Break out after making the 10 shortest connections
        if conn_count >= 1000 {
            break;
        }
    }

    println!(
        "{:?}",
        circuits
            .iter()
            .sorted_by(|a, b| b.len().cmp(&a.len()))
            .take(3)
            .fold(1, |acc, c| acc * c.len())
    );
}

fn sol2(lines: io::Lines<io::BufReader<File>>) {
    let positions: Vec<Vec<i64>> = lines
        .map(|x| {
            x.unwrap()
                .split(",")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let dist_lookup: HashMap<(usize, usize), f64> = precompute_distances(&positions);

    let mut circuits: Vec<HashSet<usize>> = vec![];
    for ((i, j), _) in dist_lookup
        .clone()
        .iter()
        .sorted_by(|(_, v1), (_, v2)| v1.total_cmp(v2))
    {
        let mut idx_to_extend = 0;
        let mut idx_to_delete = 0;
        let mut extend_flag = false;
        let mut delete_flag = false;
        for (idx, c) in circuits.iter().enumerate() {
            if c.contains(i) || c.contains(j) {
                if extend_flag {
                    delete_flag = true;
                    idx_to_delete = idx;
                } else {
                    extend_flag = true;
                    idx_to_extend = idx;
                }
            }
        }

        if delete_flag {
            let new_circuit: HashSet<usize> = circuits
                .get(idx_to_extend)
                .unwrap()
                .union(circuits.get(idx_to_delete).unwrap())
                .map(|x| *x)
                .collect();
            circuits.remove(idx_to_delete);
            circuits.remove(idx_to_extend);
            circuits.push(new_circuit);
        } else if extend_flag {
            let mut new_circuit = circuits.get(idx_to_extend).unwrap().clone();
            new_circuit.insert(*i);
            new_circuit.insert(*j);
            circuits.remove(idx_to_extend);
            circuits.push(new_circuit);
        } else {
            circuits.push(HashSet::from([*i, *j]))
        }
        // Break out after making the 10 shortest connections
        if circuits.len() == 1 && circuits.get(0).unwrap().iter().len() == 1000 {
            println!("{:?} {:?}", positions[*i], positions[*j]);
            println!("{}", positions[*i][0] * positions[*j][0]);
            break;
        }
    }
}
// ===========================================================================
