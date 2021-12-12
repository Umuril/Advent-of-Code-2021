use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sample_path_part1 = "sample1.txt";
    println!(
        "Solution of part1 with {}: {}",
        sample_path_part1,
        solve(sample_path_part1, 1)
    );

    let input_path_part1 = "input1.txt";
    println!(
        "Solution of part1 with {}: {}",
        input_path_part1,
        solve(input_path_part1, 1)
    );

    let sample_path_part2 = "sample2.txt";
    println!(
        "Solution of part2 with {}: {}",
        sample_path_part2,
        solve(sample_path_part2, 3)
    );

    let input_path_part2 = "input2.txt";
    println!(
        "Solution of part2 with {}: {}",
        input_path_part2,
        solve(input_path_part2, 3)
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