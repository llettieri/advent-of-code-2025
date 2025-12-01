use std::fs;

const MOD: u32 = 100;

fn rotate_right(current: u32, rotation: u32) -> u32 {
    (current + rotation) % MOD
}

fn rotate_left(current: u32, rotation: u32) -> u32 {
    (current + MOD - (rotation % MOD)) % MOD
}

/// Returns (number_of_zero_hits_during_rotation, new_current)
fn count_zero_hits_and_advance(current: u32, rotation: u32, direction: char) -> (u32, u32) {
    // first positive rotation in [1..100]
    let mut first_rotation_result = match direction {
        'R' => (MOD - (current % MOD)) % MOD,
        'L' => current % MOD,
        _ => return (0, current),
    };

    first_rotation_result = if first_rotation_result == 0 {
        MOD
    } else {
        first_rotation_result
    }; // 0 means 100th click
    let zeros = if first_rotation_result > rotation {
        0
    } else {
        1 + (rotation - first_rotation_result) / MOD
    };

    let new_current = match direction {
        'R' => rotate_right(current, rotation),
        'L' => rotate_left(current, rotation),
        _ => current,
    };

    (zeros, new_current)
}
fn decode_file(include_rotation_hits: bool) -> u32 {
    let mut current_value: u32 = 50;
    let mut zero_count: u32 = 0;

    let rotation_instructions =
        fs::read_to_string("day-1/code.txt").expect("Code file needs to be present!");

    for line in rotation_instructions.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let rotation_count: u32 = chars.as_str().parse().expect("Invalid rotation number");

        if include_rotation_hits {
            let (hits, new_current) =
                count_zero_hits_and_advance(current_value, rotation_count, direction);
            zero_count += hits;
            current_value = new_current;
        } else {
            match direction {
                'R' => current_value = rotate_right(current_value, rotation_count),
                'L' => current_value = rotate_left(current_value, rotation_count),
                _ => continue,
            }

            if current_value == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}

fn main() {
    let code_1 = decode_file(false);
    let code_2 = decode_file(true);

    println!("Passcode 1 is: {}", code_1);
    println!("Passcode 2 is: {}", code_2);
}
