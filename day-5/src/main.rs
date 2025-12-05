use std::fs;

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    end: i64,
}

fn parse_database() -> (String, Vec<i64>) {
    let ingredients_db =
        fs::read_to_string("day-5/ingredients_db.txt").expect("No ingredients_db.txt file found!");
    let db_parts: Vec<&str> = ingredients_db.split("\n\n").collect();

    if db_parts.len() < 2 {
        panic!("No ingredients found!");
    }

    (
        db_parts[0].to_string(),
        db_parts[1].lines().map(|id| id.parse().unwrap()).collect(),
    )
}

fn is_ingredient_fresh(ingredient_id: &i64, fresh_ingredient_id_ranges: &String) -> bool {
    for range_line in fresh_ingredient_id_ranges.lines() {
        let range: Vec<i64> = range_line
            .split("-")
            .map(|id| id.parse().unwrap())
            .collect();

        // Safety net
        if range.len() != 2 {
            continue;
        }

        let (start, end) = (&range[0], &range[1]);

        if ingredient_id >= start && ingredient_id <= end {
            return true;
        }
    }

    false
}

fn get_expected_fresh_ingredients_count(fresh_ingredient_id_ranges: &String) -> i64 {
    let mut ranges: Vec<Range> = vec![];

    for range_line in fresh_ingredient_id_ranges.lines() {
        let range: Vec<i64> = range_line
            .split("-")
            .map(|id| id.parse().unwrap())
            .collect();

        // Safety net
        if range.len() != 2 {
            continue;
        }

        let (start, end) = (range[0], range[1]);

        ranges.push(Range { start, end });
    }

    // Sort by start and merge overlaps
    ranges.sort_unstable_by_key(|r| r.start);

    let mut unique_ranges: Vec<Range> = Vec::new();
    for range in ranges {
        if let Some(last) = unique_ranges.last_mut() {
            if range.start <= last.end + 1 {
                last.end = last.end.max(range.end); // Merge
            } else {
                unique_ranges.push(range);
            }
        } else {
            unique_ranges.push(range);
        }
    }


    unique_ranges.iter().map(|r| r.end - r.start + 1).sum()
}

fn main() {
    let (fresh_ingredient_id_ranges, ingredient_ids) = parse_database();
    let mut available_fresh_ingredients: Vec<i64> = vec![];

    for ingredient_id in ingredient_ids {
        if is_ingredient_fresh(&ingredient_id, &fresh_ingredient_id_ranges) {
            available_fresh_ingredients.push(ingredient_id);
        }
    }

    println!(
        "There are {} fresh ingredients",
        available_fresh_ingredients.len()
    );

    let expected_fresh_ingredients_count: i64 =
        get_expected_fresh_ingredients_count(&fresh_ingredient_id_ranges);

    println!(
        "{} are expected to be fresh ingredients",
        expected_fresh_ingredients_count
    );
}
