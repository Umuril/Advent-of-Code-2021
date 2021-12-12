use std::cmp::{max_by, max_by_key, min_by_key};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::id;

fn main() {
    let sample_path = "src/sample.txt";
    println!(
        "Solution of part1 with {}: {}",
        sample_path,
        solve_part1(sample_path, 5)
    );

    let input_path = "src/input.txt";
    println!(
        "Solution of part1 with {}: {}",
        input_path,
        solve_part1(input_path, 12)
    );

    println!(
        "Solution of part2 with {}: {}",
        sample_path,
        solve_part2(sample_path, 5)
    );

    println!(
        "Solution of part2 with {}: {}",
        input_path,
        solve_part2(input_path, 12)
    );
}

fn solve_part1(path: &str, size: usize) -> u32 {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines();

    let mut sum = vec![0; size];

    while let Some(Ok(line)) = lines.next() {
        for (idx, c) in line.chars().enumerate() {
            match c {
                '0' => {
                    sum[idx] -= 1;
                }
                '1' => {
                    sum[idx] += 1;
                }
                _ => {}
            }
        }
    }

    let mut gamma = 0u32;
    let mut epsilon = 0u32;

    for (idx, s) in sum.iter().enumerate() {
        let inc = 1 << (size - idx - 1);
        if s >= &0 {
            gamma += inc;
        } else {
            epsilon += inc;
        }
    }

    gamma * epsilon
}

fn solve_part2(path: &str, size: usize) -> u32 {
    let mut lines = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut max = lines.clone();
    let mut min = lines.clone();

    for i in 0..size {
        let mut max_zeros = vec![];
        let mut max_ones = vec![];

        for line in max {
            if line.chars().nth(i).unwrap() == '0' {
                max_zeros.push(line);
            } else {
                max_ones.push(line);
            }
        }

        let mut min_zeros = vec![];
        let mut min_ones = vec![];

        for line in min {
            if line.chars().nth(i).unwrap() == '0' {
                min_zeros.push(line);
            } else {
                min_ones.push(line);
            }
        }

        max = max_by_key(max_zeros, max_ones, |x| x.len());

        min = match (min_zeros.len(), min_ones.len()) {
            (0, _) => min_ones,
            (_, 0) => min_zeros,
            (a, b) if a > b => min_ones,
            (_, _) => min_zeros,
        };
    }

    let mut gamma = u32::from_str_radix(&max.remove(0), 2).unwrap();
    let mut epsilon = u32::from_str_radix(&min.remove(0), 2).unwrap();

    gamma * epsilon
}
