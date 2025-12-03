use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct NumberTracker {
    num: i32,
    pos: usize,
}

#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<i32>,
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let batteries: Vec<i32> = value
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect();

        Self { batteries }
    }
}

fn get_highest_jolts_combo_p1(battery_bank: &BatteryBank) -> i32 {
    let mut highest = NumberTracker {
        num: battery_bank.batteries[0],
        pos: 0,
    };

    // get highest number
    for i in 1..battery_bank.batteries.len() - 1 {
        let battery_num = battery_bank.batteries[i];
        if battery_num > highest.num {
            highest = NumberTracker {
                num: battery_num,
                pos: i,
            };
        }
    }

    let mut second_highest = NumberTracker {
        num: battery_bank.batteries[highest.pos + 1],
        pos: highest.pos + 1,
    };

    // get second highest number
    for i in highest.pos + 2..battery_bank.batteries.len() {
        let battery_num = battery_bank.batteries[i];
        if battery_num > second_highest.num {
            second_highest = NumberTracker {
                num: battery_num,
                pos: i,
            };
        }
    }

    highest.num * 10 + second_highest.num
}

fn get_highest_jolts_combo_p2(battery_bank: &BatteryBank) -> u64 {
    let mut removals_remaining = battery_bank.batteries.len() - 12;
    let mut stack: Vec<i32> = Vec::new();

    for battery in &battery_bank.batteries {
        while let Some(&top) = stack.last() {
            if removals_remaining == 0 || *battery <= top {
                break;
            }

            stack.pop();
            removals_remaining -= 1;
        }

        stack.push(*battery);
    }

    if removals_remaining > 0 {
        for _ in 0..removals_remaining {
            stack.pop();
        }
    }

    stack
        .iter()
        .map(|num| num.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn solve(input: &str) -> SolutionPair {
    let battery_banks: Vec<BatteryBank> = input.lines().map(BatteryBank::from).collect();
    let sol1: i32 = battery_banks.iter().map(get_highest_jolts_combo_p1).sum();
    let sol2: u64 = battery_banks.iter().map(get_highest_jolts_combo_p2).sum();
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day3_p1() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "357");
    }

    #[test]
    fn test_example_input_day3_p2() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "3121910778619");
    }
}
