use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sample_path = "src/sample.txt";
    println!(
        "Solution of part1 with {}: {}",
        sample_path,
        solve(sample_path, false)
    );

    let input_path = "src/input.txt";
    println!(
        "Solution of part1 with {}: {}",
        input_path,
        solve(input_path, false)
    );

    println!(
        "Solution of part2 with {}: {}",
        sample_path,
        solve(sample_path, true)
    );

    println!(
        "Solution of part2 with {}: {}",
        input_path,
        solve(input_path, true)
    );
}

fn solve(path: &str, with_aim: bool) -> u32 {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines();

    let mut horizontal = 0u32;
    let mut depth = 0u32;
    let mut aim = 0u32;

    while let Some(Ok(line)) = lines.next() {
        if let Some((cmd, v)) = line.split_once(' ') {
            let value = v.parse::<u32>().unwrap();
            match cmd {
                "forward" => {
                    horizontal += value;
                    if with_aim {
                        aim += depth * value;
                    }
                }
                "up" => {
                    depth -= value;
                }
                "down" => {
                    depth += value;
                }
                _ => {}
            }
        }
    }

    if with_aim {
        horizontal * aim
    } else {
        horizontal * depth
    }
}
