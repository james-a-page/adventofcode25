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
fn collect_grid(lines: Vec<String>) -> Vec<Vec<String>> {
    let y: usize = lines.len();
    let mut grid: Vec<Vec<String>> = vec![vec![]; y];
    for (y, line) in lines.iter().enumerate() {
        grid[y] = line
            .split(" ")
            .map(|x| x.to_string())
            .filter(|x| x != "")
            .collect();
    }
    return grid;
}

fn collect_grid2(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut max_len = 0;
    let y: usize = lines.len();
    let mut grid: Vec<Vec<String>> = vec![vec![]; y];
    for (y, line) in lines.iter().enumerate() {
        if max_len < line.len() {
            max_len = line.len();
        }
        grid[y] = line.chars().map(|x| x.to_string()).collect();
    }
    let mut padded_grid: Vec<Vec<String>> = vec![vec![]; y];
    for (i, mut row) in grid.into_iter().enumerate() {
        row.resize(max_len, " ".to_string());
        padded_grid[i] = row;
    }
    return padded_grid;
}

fn transpose_grid(grid: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let cols = grid[0].len();

    let mut iters: Vec<_> = grid.into_iter().map(|n| n.into_iter()).collect();
    (0..cols)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<String>>()
        })
        .collect()
}

fn sol1(lines: io::Lines<io::BufReader<File>>) {
    let grid = collect_grid(lines.map(|x| x.unwrap()).collect());
    let answers = transpose_grid(grid)
        .into_iter()
        .map(|mut row| {
            let op = row.pop().expect("");

            let nums = row.into_iter().filter_map(|x| x.parse::<usize>().ok());

            match op.as_str() {
                "*" => nums.fold(1, |acc, n| acc * n),
                "+" => nums.fold(0, |acc, n| acc + n),
                _ => 0,
            }
        })
        .collect::<Vec<usize>>();
    let total: usize = answers.iter().sum();
    println!("{}", total)
}

fn sol2(lines: io::Lines<io::BufReader<File>>) {
    let mut grid = collect_grid2(lines.map(|x| x.unwrap()).collect());

    let operations: Vec<String> = grid
        .pop()
        .expect("")
        .into_iter()
        .filter(|x| x == "+" || x == "*")
        .collect();

    let collapsed_nums: Vec<String> = transpose_grid(grid)
        .into_iter()
        .map(|split_num| split_num.join("").replace(" ", ""))
        .collect();

    let total: usize = collapsed_nums
        .split(|x| x == "")
        .into_iter()
        .zip(operations)
        .map(|(nums, op)| {
            let vals = nums.iter().map(|x| x.parse::<usize>().expect("msg"));
            match op.as_str() {
                "*" => vals.fold(1, |acc, n| acc * n),
                "+" => vals.fold(0, |acc, n| acc + n),
                _ => 0,
            }
        })
        .sum();

    println!("{:?}", total)
}
// ===========================================================================
