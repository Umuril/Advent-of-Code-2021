use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sample_path = "src/sample.txt";
    println!(
        "Solution of part1 with {}: {}",
        sample_path,
        solve_part1(sample_path)
    );

    let input_path = "src/input.txt";
    println!(
        "Solution of part1 with {}: {}",
        input_path,
        solve_part1(input_path)
    );

    println!(
        "Solution of part2 with {}: {}",
        sample_path,
        solve_part2(sample_path)
    );

    println!(
        "Solution of part2 with {}: {}",
        input_path,
        solve_part2(input_path)
    );
}

#[derive(Debug)]
struct Board {
    cells: [[i32; 5]; 5],
    rows: [u32; 5],
    cols: [u32; 5],
    win: bool,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [[0; 5]; 5],
            rows: [5; 5],
            cols: [5; 5],
            win: false,
        }
    }

    fn from_vecs(board: Vec<Vec<u32>>) -> Self {
        let mut cells = [[0; 5]; 5];

        for (row_idx, row) in board.into_iter().enumerate() {
            for (col_idx, val) in row.into_iter().enumerate() {
                cells[row_idx][col_idx] = val as i32;
            }
        }

        Board {
            cells,
            rows: [5; 5],
            cols: [5; 5],
            win: false,
        }
    }

    fn mark(&mut self, val: u32) {
        for (row_idx, row) in self.cells.into_iter().enumerate() {
            for (col_idx, cell) in row.into_iter().enumerate() {
                if cell == val as i32 {
                    self.cells[row_idx][col_idx] = -1;
                    self.rows[row_idx] -= 1;
                    self.cols[col_idx] -= 1;

                    if self.rows[row_idx] == 0 || self.cols[col_idx] == 0 {
                        self.win = true;
                    }

                    return;
                }
            }
        }
        return;
    }

    fn sum_unmarked(&self) -> u32 {
        let mut acc = 0;

        for row in self.cells {
            for cell in row {
                if cell >= 0 {
                    acc += cell as u32;
                }
            }
        }

        acc
    }
}

fn solve_part1(path: &str) -> u32 {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines();

    let nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut rows = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let row = line
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        rows.push(row);
    }

    let mut boards = rows
        .chunks(5)
        .map(|v| v.into())
        .map(|x| Board::from_vecs(x))
        .collect::<Vec<_>>();

    for &num in nums.iter() {
        for board in boards.iter_mut() {
            board.mark(num);

            if board.win {
                return board.sum_unmarked() * num;
            }
        }
    }

    0
}

fn solve_part2(path: &str) -> u32 {
    let mut lines = BufReader::new(File::open(path).unwrap()).lines();

    let nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut rows = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let row = line
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        rows.push(row);
    }

    let mut boards = rows
        .chunks(5)
        .map(|v| v.into())
        .map(|x| Board::from_vecs(x))
        .collect::<Vec<_>>();

    for &num in nums.iter() {
        let mut missing_boards = boards
            .iter_mut()
            .filter(|x| !x.win)
            .enumerate()
            .collect::<Vec<_>>();

        let siz = missing_boards.len();

        for (_, board) in missing_boards.iter_mut() {
            board.mark(num);

            if siz == 1 && board.win {
                return board.sum_unmarked() * num;
            }
        }
    }

    0
}
