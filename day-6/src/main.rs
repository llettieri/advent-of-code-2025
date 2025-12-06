use std::fs;

fn parse_number_row(row: &str) -> Vec<u64> {
    row.split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn parse_operation_row(row: &str) -> Vec<String> {
    row.split_whitespace()
        .map(|operation| String::from(operation))
        .collect()
}

/// Parse the math problems formatted in rows of whitespace-separated numbers,
fn split_whitespace_columns(math_problems: &str) -> (Vec<Vec<u64>>, Vec<String>) {
    let rows: Vec<&str> = math_problems.lines().map(|line| line.trim()).collect();

    let operations = parse_operation_row(rows[rows.len() - 1]);
    let number_rows = &rows[..rows.len() - 1];

    let columns = if number_rows.is_empty() {
        0
    } else {
        parse_number_row(number_rows[0]).len()
    };

    let mut problems: Vec<Vec<u64>> = vec![Vec::new(); columns];

    for row in number_rows {
        let numbers = parse_number_row(row);

        for (index, &number) in numbers.iter().enumerate() {
            problems[index].push(number);
        }
    }

    (problems, operations)
}

/// Parse the cephalopod-style fixed-width digit columns (right-to-left)
fn split_digit_columns(math_problems: &str) -> (Vec<Vec<u64>>, Vec<String>) {
    let rows: Vec<&str> = math_problems.lines().collect();

    let operations_row = rows[rows.len() - 1];
    let number_rows = &rows[..rows.len() - 1];

    // Pad rows to the same width using bytes for fast indexed access
    let max_row_len = number_rows.iter().map(|row| row.len()).max().unwrap_or(0);
    let padded_number_rows: Vec<Vec<u8>> = number_rows
        .iter()
        .map(|row| {
            let mut byte = row.as_bytes().to_vec();
            byte.resize(max_row_len, b' ');
            byte
        })
        .collect();

    // Separator columns are columns where every row has a space
    let mut is_separator = vec![false; max_row_len];
    for i in 0..max_row_len {
        is_separator[i] = padded_number_rows.iter().all(|row| row[i] == b' ');
    }

    // Group contiguous non-separator columns
    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;

    while i < max_row_len {
        if is_separator[i] {
            i += 1;
            continue;
        }
        let start = i;

        while i < max_row_len && !is_separator[i] {
            i += 1;
        }
        groups.push((start, i - 1));
    }

    // Prepare padded operations row
    let mut operations_bytes = operations_row.as_bytes().to_vec();
    operations_bytes.resize(max_row_len, b' ');

    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<String> = Vec::new();

    for (start, end) in groups.iter() {
        let mut operands: Vec<u64> = Vec::new();

        for col in *start..=*end {
            let mut digits = String::new();

            for row in &padded_number_rows {
                let digit = row[col];
                if digit.is_ascii_digit() {
                    digits.push(digit as char);
                }
            }

            if !digits.is_empty() {
                operands.push(digits.parse().expect("Failed to parse cephalopod number"));
            }
        }

        // Find operator within group's columns (first non-space)
        let operation_char = (*start..=*end)
            .find_map(|index| {
                let byte = operations_bytes[index];
                if byte != b' ' { Some(byte as char) } else { None }
            })
            .unwrap_or('+');

        problems.push(operands);
        operations.push(operation_char.to_string());
    }

    // cephalopod sheets are read right-to-left
    problems.reverse();
    operations.reverse();

    (problems, operations)
}

fn transform_problems_into_columns(
    math_problems: &str,
    split_by_digits: bool,
) -> (Vec<Vec<u64>>, Vec<String>) {
    if split_by_digits {
        split_digit_columns(math_problems)
    } else {
        split_whitespace_columns(math_problems)
    }
}

fn calculate_grand_total(problems: &Vec<Vec<u64>>, operations: &Vec<String>) -> u64 {
    problems
        .iter()
        .enumerate()
        .map(|(i, p)| match operations[i].as_str() {
            "+" => p.iter().sum::<u64>(),
            "*" => p.iter().product::<u64>(),
            _ => 0,
        })
        .sum()
}

fn main() {
    let math_problems =
        fs::read_to_string("day-6/math_problems.txt").expect("File math_problems.txt not found!");

    let (problems, operations) = transform_problems_into_columns(&math_problems, false);
    let grand_total = calculate_grand_total(&problems, &operations);

    let (problems_ceph, operations_ceph) = transform_problems_into_columns(&math_problems, true);
    let grand_total_cephalopod = calculate_grand_total(&problems_ceph, &operations_ceph);

    println!("The grand total is: {}", grand_total);
    println!(
        "The grand total with cephalopod math is: {}",
        grand_total_cephalopod
    );
}
