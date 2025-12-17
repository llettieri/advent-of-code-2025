use std::collections::HashSet;
use std::fs;

fn count_splits(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let start_col = grid[0]
        .iter()
        .position(|&char| char == 'S')
        .expect("No S found in first row!");

    let mut beams: Vec<usize> = vec![start_col];
    let mut current_row: usize = 0;
    let mut total_splits: usize = 0;

    while !beams.is_empty() {
        let next_row = current_row + 1;
        if next_row >= height {
            break;
        }

        let mut split_cols: HashSet<usize> = HashSet::new();
        let mut straight_cols: HashSet<usize> = HashSet::new();

        for col in beams.drain(..) {
            if col >= width {
                continue;
            }

            match grid[next_row][col] {
                '.' | 'S' => {
                    straight_cols.insert(col);
                }
                '^' => {
                    split_cols.insert(col);
                }
                _ => {}
            }
        }

        total_splits += split_cols.len();

        let mut next_beams: Vec<usize> = Vec::new();

        for &c in &split_cols {
            if c > 0 {
                next_beams.push(c - 1);
            }
            if c + 1 < width {
                next_beams.push(c + 1);
            }
        }

        // From straight beams: same column
        for &c in &straight_cols {
            next_beams.push(c);
        }

        beams = next_beams;
        current_row = next_row;
    }

    total_splits
}

fn count_possible_timelines(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    // Find S in first row
    let start_col = grid[0]
        .iter()
        .position(|&char| char == 'S')
        .expect("No S found in first row!");

    // dp[row][col] = number of timelines at cell (r, c)
    let mut display = vec![vec![0; width]; height];
    display[0][start_col] = 1;

    for row in 0..height {
        for col in 0..width {
            let ways = display[row][col];

            if ways == 0 {
                continue;
            }

            // Next row down
            let next_row = row + 1;
            if next_row >= height {
                continue;
            }

            match grid[next_row][col] {
                '.' | 'S' => {
                    display[next_row][col] += ways;
                }
                '^' => {
                    if col > 0 {
                        display[next_row][col - 1] += ways;
                    }
                    if col + 1 < width {
                        display[next_row][col + 1] += ways;
                    }
                }
                _ => {
                    // Treat other chars as blocking
                }
            }
        }
    }

    let mut timelines = 0;

    // Timelines that end by stepping below the last row:
    for c in 0..width {
        timelines += display[height - 1][c];
    }

    // Timelines that would step sideways out of bounds from any row:
    for row in 0..height {
        // If the particle is at col 0 and the next cell is a splitter, left branch leaves.
        if display[row][0] > 0 && row + 1 < height && grid[row + 1][0] == '^' {
            timelines += display[row][0]; // left branch lost
        }
        // Similarly for the rightmost column
        if display[row][width - 1] > 0 && row + 1 < height && grid[row + 1][width - 1] == '^' {
            timelines += display[row][width - 1]; // right branch lost
        }
    }

    timelines
}

fn main() {
    let manifold_diagram =
        fs::read_to_string("../manifold_diagram.txt").expect("No manifold_diagram.txt file found!");
    let grid: Vec<Vec<char>> = manifold_diagram
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let total_splits = count_splits(&grid);
    let total_timelines = count_possible_timelines(&grid);

    println!("Total splits: {}", total_splits);
    println!("Total possible timelines: {}", total_timelines);
}
