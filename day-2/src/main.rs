use regex::Regex;
use std::fs;

fn is_repeated_digit_pattern(product_id: &str, max_twice: bool) -> bool {
    let product_id_length = product_id.len();

    if max_twice {
        if product_id_length <= 1 {
            return false;
        }

        let half = product_id_length / 2;
        let first = &product_id[..half];
        let second = &product_id[half..];

        return first == second;
    }

    if product_id_length <= 1 {
        return false;
    }

    let test_value = format!("{product_id}{product_id}");
    test_value[1..test_value.len() - 1].contains(product_id)
}

fn calculate_code(max_twice: bool) -> u64 {
    let mut final_code = 0;
    let matching_re = Regex::new(r"^\d+$").unwrap();
    let id_ranges =
        fs::read_to_string("day-2/id-ranges.txt").expect("No id-ranges.txt file found!");

    for id_range in id_ranges.split(",") {
        let bounds: Vec<&str> = id_range.split("-").collect();

        let lower: u64 = bounds[0].parse().unwrap();
        let upper: u64 = bounds[1].parse().unwrap();

        // Note: upper+1 to include the upper bound in the iteration
        for i in lower..upper + 1 {
            let product_id = i.to_string();

            if matching_re.is_match(&product_id)
                && is_repeated_digit_pattern(&product_id, max_twice)
            {
                final_code += i;
                println!("Checking Product ID: {}... ", i);
            }
        }
    }

    final_code
}

fn main() {
    let final_code_1: u64 = calculate_code(true);
    let final_code_2: u64 = calculate_code(false);

    println!("Max. 2 repetitions - Final code: {}", final_code_1);
    println!("At least 2 repetitions - Final code: {}", final_code_2);
}
