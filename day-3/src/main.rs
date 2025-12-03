use std::fs;

fn get_highest_bank_joltage(bank: &str, mut battery_count: u16) -> u64 {
    let mut highest_joltage = 0;
    let bank_size = bank.len();
    let batteries = bank.chars().collect::<Vec<_>>();

    // Safety-net to prevent issues
    if battery_count == 0 {
        return 0;
    }

    if battery_count as usize > bank_size {
        battery_count = bank_size as u16
    }

    match battery_count {
        2 => {
            for i in 0..bank_size {
                let current_digit = batteries[i];

                for digit in bank[i + 1..].chars() {
                    let joltage = format!("{}{}", current_digit, digit).parse().unwrap();

                    if joltage > highest_joltage {
                        highest_joltage = joltage
                    }
                }
            }
        }
        _ => {
            // Use an algorithm to choose the lexicographically
            // Largest subsequence of length `battery_count` while preserving order
            let battery_count = battery_count as usize;

            let mut to_remove = bank_size.saturating_sub(battery_count);
            let mut stack: Vec<char> = Vec::with_capacity(bank_size);

            for &ch in &batteries {
                while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < ch {
                    stack.pop();
                    to_remove -= 1;
                }
                stack.push(ch);
            }

            // If we still need to remove characters, pop from the end
            while to_remove > 0 {
                stack.pop();
                to_remove -= 1;
            }

            // Take the first battery_count characters from the stack as the chosen subsequence
            let selected: String = stack.iter().take(battery_count).cloned().collect();

            highest_joltage = selected.parse().unwrap();
        }
    }

    highest_joltage
}

fn main() {
    let mut total_joltage_out_of_2_batteries = 0;
    let mut total_joltage_out_of_12_batteries = 0;

    let banks_of_batteries =
        fs::read_to_string("day-3/banks_of_batteries.txt").expect("No banks_of_batteries.txt file found!");

    for bank in banks_of_batteries.lines() {
        total_joltage_out_of_2_batteries += get_highest_bank_joltage(bank, 2);
        total_joltage_out_of_12_batteries += get_highest_bank_joltage(bank, 12);
    }

    println!(
        "The total joltage of 2 batteries is: {}",
        total_joltage_out_of_2_batteries
    );
    println!(
        "The total joltage of 12 batteries is: {}",
        total_joltage_out_of_12_batteries
    );
}
