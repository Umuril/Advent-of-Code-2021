use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sample_path = "sample.txt";
    println!(
        "Solution of part1 with {}: {}",
        sample_path,
        solve(sample_path, 1)
    );

    let input_path = "input.txt";
    println!(
        "Solution of part1 with {}: {}",
        input_path,
        solve(input_path, 1)
    );

    println!(
        "Solution of part2 with {}: {}",
        sample_path,
        solve(sample_path, 3)
    );

    println!(
        "Solution of part2 with {}: {}",
        input_path,
        solve(input_path, 3)
    );
}

fn solve(path: &str, n: usize) -> u32 {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines();

    let mut tmp = (&mut lines)
        .take(n)
        .map(|x| x.unwrap())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut acc = 0;
    for line in lines {
        if let Ok(value) = line {
            let parsed = value.parse::<u32>().unwrap();
            if parsed > tmp.remove(0) {
                acc += 1;
            }
            tmp.push(parsed);
        }
    }

    acc
}
