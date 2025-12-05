use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{array, env};

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

// Solutions
// ===========================================================================
fn count_surrounding(i: usize, j: usize, grid: &Vec<Vec<bool>>) -> usize {
    let y_max = grid.len() as isize;
    let x_max = grid[0].len() as isize;

    let locs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut total = 0;
    for (dy, dx) in locs {
        let ny = i as isize + dy;
        let nx = j as isize + dx;
        if nx >= 0 && nx < x_max && ny >= 0 && ny < y_max {
            if grid[ny as usize][nx as usize] {
                total += 1
            }
        }
    }
    return total;
}

fn sol1(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    let lines_vec: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let mut x: usize = 0;
    let y: usize = lines_vec.len();
    let mut grid: Vec<Vec<bool>> = vec![vec![]; y];
    for (y, line) in lines_vec.iter().enumerate() {
        grid[y] = line.chars().map(|x| x == '@').collect();
        x = line.len();
    }
    for i in 0..y {
        for j in 0..x {
            if grid[i][j] && count_surrounding(i, j, &grid) < 4 {
                total += 1;
            }
            if grid[i][j] {
                print!("{}", count_surrounding(i, j, &grid));
            } else {
                print!(".")
            }
        }
        println!("")
    }
    println!("{:?}", total);
}

fn sol2(lines: io::Lines<io::BufReader<File>>) {
    let mut total = 0;
    let lines_vec: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let mut x: usize = 0;
    let y: usize = lines_vec.len();
    let mut grid: Vec<Vec<bool>> = vec![vec![]; y];
    for (y, line) in lines_vec.iter().enumerate() {
        grid[y] = line.chars().map(|x| x == '@').collect();
        x = line.len();
    }
    let mut collected = -1;
    while collected != 0 {
        collected = 0;
        for i in 0..y {
            for j in 0..x {
                if grid[i][j] && count_surrounding(i, j, &grid) < 4 {
                    grid[i][j] = false;
                    collected += 1;
                }
            }
        }
        total += collected
    }
    println!("{:?}", total);
}
// ===========================================================================
