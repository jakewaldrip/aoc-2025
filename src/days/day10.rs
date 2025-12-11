use std::iter::zip;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Machine {
    target: String,
    buttons: Vec<String>,
    joltage: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(s: &str) -> Self {
        let start = s.find('[').unwrap();
        let end = s.find(']').unwrap();
        let target_str = &s[start + 1..end];
        let target: String = target_str
            .chars()
            .map(|c| if c == '#' { '1' } else { '0' })
            .collect();

        let after_target = &s[end + 1..];
        let jolt_start = after_target.find('{').unwrap();
        let buttons_str = &after_target[..jolt_start].trim();

        let buttons: Vec<String> = buttons_str
            .split_whitespace()
            .map(|part| {
                let inner = &part[1..part.len() - 1];
                let positions: Vec<usize> = inner
                    .split(',')
                    .map(|n| n.trim().parse().unwrap())
                    .collect();
                let len = target.len();
                let mut bin = vec!['0'; len];
                for &p in &positions {
                    if p < len {
                        bin[p] = '1';
                    }
                }
                bin.into_iter().collect()
            })
            .collect();

        let jolt_str = &after_target[jolt_start + 1..after_target.len() - 1];
        let joltage: Vec<usize> = jolt_str
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect();

        Machine {
            target,
            buttons,
            joltage,
        }
    }
}

pub fn calc_all_subsets(strings: &[String]) -> Vec<Vec<String>> {
    let n = strings.len();
    let mut result = Vec::new();

    for mask in 1..(1 << n) {
        let mut subset = Vec::new();
        for (j, val) in strings.iter().enumerate() {
            if (mask & (1 << j)) != 0 {
                subset.push(val.clone());
            }
        }
        result.push(subset);
    }
    result
}

fn binary_xor(x: &str, y: &str) -> String {
    zip(x.chars(), y.chars())
        .map(|(x, y)| ((x as u8 - b'0') ^ ((y as u8 - b'0') + b'0')) as char)
        .collect()
}

fn get_lowest_button_presses_p1(machine: &Machine) -> i32 {
    let subsets = calc_all_subsets(&machine.buttons);
    let min_presses = subsets
        .iter()
        .filter_map(|candidate| {
            let mut current_machine_state = "0".repeat(machine.target.len()).to_string();
            for button_presses in candidate {
                current_machine_state = binary_xor(&current_machine_state, button_presses);
            }

            if current_machine_state == machine.target {
                return Some(candidate.len());
            }

            None
        })
        .min()
        .unwrap();

    min_presses as i32
}

pub fn solve(input: &str) -> SolutionPair {
    let machines: Vec<Machine> = input.lines().map(|line| line.into()).collect();

    // part 1
    let sol1: i32 = machines.iter().map(get_lowest_button_presses_p1).sum();

    // part 2
    let sol2 = 0;
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day10_p1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "7");
    }

    #[test]
    fn test_binary_xor() {
        let x = "0110";
        let y = "1111";
        let result = binary_xor(x, y);
        assert_eq!(result, "1001");
    }

    #[test]
    fn test_three_elements() {
        let input = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let mut result = calc_all_subsets(&input);
        println!("All subsets\n{result:#?}");
        result.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
        let expected = vec![
            vec!["a".to_string()],
            vec!["b".to_string()],
            vec!["c".to_string()],
            vec!["a".to_string(), "b".to_string()],
            vec!["a".to_string(), "c".to_string()],
            vec!["b".to_string(), "c".to_string()],
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        ];
        assert_eq!(result, expected);
    }
}
