use std::fs;

const PAPER_ROLL: char = '@';

fn get_accessible_paper_rolls(
    grid: &Vec<Vec<char>>,
    directions: &Vec<(i32, i32)>,
) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut accessible: Vec<(usize, usize)> = vec![];

    for row in 0..height {
        for col in 0..width {
            if grid[row][col] != PAPER_ROLL {
                continue;
            }

            let mut count = 0;

            for (relative_col, relative_row) in directions {
                let (neighbor_col, neighbor_row) = (col as i32 + relative_col, row as i32 + relative_row);

                let neighbor_has_valid_y: bool = 0 <= neighbor_row && neighbor_row < (height as i32);
                let neighbor_has_valid_x: bool = 0 <= neighbor_col && neighbor_col < (width as i32);

                if neighbor_has_valid_y && neighbor_has_valid_x {
                    let neighbor = grid[neighbor_row as usize][neighbor_col as usize];

                    if neighbor == PAPER_ROLL {
                        count += 1
                    }
                }
            }

            if count < 4 {
                accessible.push((row, col));
            }
        }
    }

    accessible
}

fn remove_paper_rolls(grid: &mut Vec<Vec<char>>, positions: &Vec<(usize, usize)>) {
    for &(row, col) in positions {
        grid[row][col] = '.';
    }
}

fn main() {
    let diagram = fs::read_to_string("day-4/diagram.txt").expect("No diagram.txt file found!");

    let mut grid: Vec<Vec<char>> = diagram.lines().map(|line| line.chars().collect()).collect(); // 2 dimensional vector

    let directions: Vec<(i32, i32)> = vec![
        (-1, -1), // NW
        (0, -1),  // N
        (1, -1),  // NE
        (-1, 0),  // W
        (1, 0),   // E
        (-1, 1),  // SW
        (0, 1),   // S
        (1, 1),   // SE
    ];

    let accessible_paper_rolls = get_accessible_paper_rolls(&grid.clone(), &directions);
    println!("Accessible paper rolls: {}", accessible_paper_rolls.len());


    let mut removed_paper_rolls = 0;

    loop {
        let accessible = get_accessible_paper_rolls(&grid, &directions);

        if accessible.is_empty() {
            break;
        }

        removed_paper_rolls += accessible.len();
        remove_paper_rolls(&mut grid, &accessible);
    }

    println!("Removed paper rolls: {}", removed_paper_rolls);
}
