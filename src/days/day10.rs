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

pub fn solve(input: &str) -> SolutionPair {
    println!("Input: \n{input}");
    let machines: Vec<Machine> = input.lines().map(|line| line.into()).collect();
    println!("Machines:\n{machines:#?}");
    let sol1 = 0;
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
}
