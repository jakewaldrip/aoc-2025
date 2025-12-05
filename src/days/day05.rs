use std::cmp::max;

use crate::{Solution, SolutionPair, utils::range::Range};

///////////////////////////////////////////////////////////////////////////////

fn parse_p1(input: &str) -> (Vec<Range>, Vec<i64>) {
    let inputs: Vec<&str> = input.split("\n\n").collect();
    let ranges: Vec<Range> = inputs[0].split('\n').map(Range::from).collect();
    let fruits: Vec<i64> = inputs[1]
        .split('\n')
        .filter(|fruit| !fruit.is_empty())
        .map(|fruit| fruit.parse().unwrap())
        .collect();
    (ranges, fruits)
}

fn merge_ranges(original_ranges: &[Range]) -> Vec<Range> {
    let mut merged_ranges: Vec<Range> = vec![original_ranges[0]];
    for original_range in original_ranges.iter().skip(1) {
        let last_range = merged_ranges.last_mut().unwrap();

        if original_range.start <= last_range.end {
            last_range.end = max(last_range.end, original_range.end);
        } else {
            merged_ranges.push(*original_range);
        }
    }

    merged_ranges
}

pub fn solve(input: &str) -> SolutionPair {
    let (mut ranges, fruits) = parse_p1(input);

    // part 1
    let sol1: usize = fruits
        .iter()
        .filter(|fruit| {
            for range in &ranges {
                if **fruit >= range.start && **fruit <= range.end {
                    return true;
                }
            }

            false
        })
        .count();

    // part 2
    ranges.sort_by_key(|range| range.start);
    let ranges = merge_ranges(&ranges);
    let sol2: i64 = ranges.iter().map(|range| range.end - range.start + 1).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day5_p1() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "3");
    }

    #[test]
    fn test_example_input_day5_p2() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "14");
    }
}
