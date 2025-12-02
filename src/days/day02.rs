use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split('-').collect();
        Self {
            start: parts[0].trim().parse().unwrap(),
            end: parts[1].trim().parse().unwrap(),
        }
    }
}

/// Each range should be made of subranges that are the same # of digits
fn split_range_into_subranges_p1(range: &Range) -> Vec<Range> {
    let mut subranges: Vec<Range> = Vec::new();
    let mut current_range: i64 = range.start;
    let end: i64 = range.end;

    while current_range <= end {
        let digits = current_range.to_string().len() as u32;
        let next_power: i64 = 10i64.pow(digits);
        let end_current_range = std::cmp::min(end, next_power - 1) as i64;

        if digits.is_multiple_of(2) {
            subranges.push(Range {
                start: current_range,
                end: end_current_range,
            });
        }

        current_range = next_power;
    }

    subranges
}

fn split_range_into_subranges_p2(range: &Range) -> Vec<Range> {
    let mut subranges: Vec<Range> = Vec::new();
    let mut current_range: i64 = range.start;
    let end: i64 = range.end;

    while current_range <= end {
        let digits = current_range.to_string().len() as u32;
        let next_power: i64 = 10i64.pow(digits);
        let end_current_range = std::cmp::min(end, next_power - 1) as i64;

        subranges.push(Range {
            start: current_range,
            end: end_current_range,
        });

        current_range = next_power;
    }

    subranges
}

fn get_sum_of_invalid_ids_in_range_p1(range: &Range) -> i64 {
    let digits = range.start.to_string().len();
    let half_sig_start: i64 = range
        .start
        .to_string()
        .chars()
        .take(digits / 2)
        .collect::<String>()
        .parse()
        .unwrap();
    let half_sig_end: i64 = range
        .end
        .to_string()
        .chars()
        .take(digits / 2)
        .collect::<String>()
        .parse()
        .unwrap();

    let mut sum: i64 = 0;
    for i in half_sig_start..=half_sig_end {
        let invalid_id: i64 = i.to_string().repeat(2).parse().unwrap();
        if invalid_id <= range.end && invalid_id >= range.start {
            sum += invalid_id;
        }
    }

    sum
}

fn get_sum_of_invalid_ids_in_range_p2(range: &Range) -> i64 {
    let mut processed_ids: HashMap<i64, bool> = HashMap::new();
    let digits = range.start.to_string().len();
    let mut sum = 0;

    for i in 0..digits / 2 + 1 {
        // Continue if we cannot make a repeat work here (not a mulitple)
        if !digits.is_multiple_of(i) {
            continue;
        }

        let multiple_sig_start: i64 = range
            .start
            .to_string()
            .chars()
            .take(i)
            .collect::<String>()
            .parse()
            .unwrap();
        let multiple_sig_end: i64 = range
            .end
            .to_string()
            .chars()
            .take(i)
            .collect::<String>()
            .parse()
            .unwrap();

        for j in multiple_sig_start..=multiple_sig_end {
            let repeat_num = digits / i;
            let invalid_id: i64 = match j.to_string().repeat(repeat_num).parse::<i64>() {
                Ok(id) => id,
                Err(_) => continue,
            };

            if invalid_id <= range.end
                && invalid_id >= range.start
                && !processed_ids.contains_key(&invalid_id)
            {
                processed_ids.insert(invalid_id, true);
                sum += invalid_id;
            }
        }
    }

    sum
}

pub fn solve(input: &str) -> SolutionPair {
    let input_ranges: Vec<Range> = input.split(',').map(Range::from).collect();
    let all_ranges_p1: Vec<Range> = input_ranges
        .iter()
        .flat_map(split_range_into_subranges_p1)
        .collect();
    let sol1: i64 = all_ranges_p1
        .iter()
        .map(get_sum_of_invalid_ids_in_range_p1)
        .sum();

    let all_ranges_p2: Vec<Range> = input_ranges
        .iter()
        .flat_map(split_range_into_subranges_p2)
        .collect();
    let sol2: i64 = all_ranges_p2
        .iter()
        .map(get_sum_of_invalid_ids_in_range_p2)
        .sum();

    // println!("P2 Ranges: {all_ranges_p2:#?}");
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day2_p1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "1227775554");
    }

    #[test]
    fn test_example_input_day2_p2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "4174379265");
    }
}
