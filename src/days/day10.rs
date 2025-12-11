use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Machine {
    target: u64,
    buttons: Vec<u64>,
    _joltage: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(s: &str) -> Self {
        let start = s.find('[').unwrap();
        let end = s.find(']').unwrap();
        let target_str = &s[start + 1..end];
        let len = target_str.len();
        let mut target: u64 = 0;
        for (i, c) in target_str.chars().enumerate() {
            if c == '#' {
                target |= 1 << i;
            }
        }

        let after_target = &s[end + 1..];
        let jolt_start = after_target.find('{').unwrap();
        let buttons_str = &after_target[..jolt_start].trim();

        let buttons: Vec<u64> = buttons_str
            .split_whitespace()
            .map(|part| {
                let inner = &part[1..part.len() - 1];
                let positions: Vec<usize> = inner
                    .split(',')
                    .map(|n| n.trim().parse().unwrap())
                    .collect();
                let mut bin: u64 = 0;
                for &p in &positions {
                    if p < len {
                        bin |= 1 << p;
                    }
                }
                bin
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
            _joltage: joltage,
        }
    }
}

pub fn calc_all_subsets(items: &[u64]) -> Vec<Vec<u64>> {
    let n = items.len();
    let mut result = Vec::new();

    for mask in 1..(1 << n) {
        let mut subset = Vec::new();
        for (j, &val) in items.iter().enumerate() {
            if (mask & (1 << j)) != 0 {
                subset.push(val);
            }
        }
        result.push(subset);
    }
    result
}

fn get_lowest_button_presses_p1(machine: &Machine) -> i32 {
    let subsets = calc_all_subsets(&machine.buttons);
    let min_presses = subsets
        .iter()
        .filter_map(|candidate| {
            let mut current_machine_state: u64 = 0;
            for &button_presses in candidate {
                current_machine_state ^= button_presses;
            }

            if current_machine_state == machine.target {
                Some(candidate.len())
            } else {
                None
            }
        })
        .min()
        .unwrap();

    min_presses as i32
}

fn get_lowest_button_presses_p2(machine: &Machine) -> i32 {
    todo!()
}

pub fn solve(input: &str) -> SolutionPair {
    let machines: Vec<Machine> = input.lines().map(|line| line.into()).collect();

    // part 1
    let sol1: i32 = machines.iter().map(get_lowest_button_presses_p1).sum();

    // part 2
    let sol2: i32 = machines.iter().map(get_lowest_button_presses_p2).sum();

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
    fn test_example_input_day10_p2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "33");
    }

    #[test]
    fn test_three_elements() {
        let input = vec![0u64, 1, 2];
        let mut result = calc_all_subsets(&input);
        println!("All subsets\n{result:#?}");
        result.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
        let expected = vec![
            vec![0u64],
            vec![1u64],
            vec![2u64],
            vec![0u64, 1],
            vec![0u64, 2],
            vec![1u64, 2],
            vec![0u64, 1, 2],
        ];
        assert_eq!(result, expected);
    }
}
