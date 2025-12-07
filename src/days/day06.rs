use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
enum Operands {
    Mult,
    Add,
}

impl From<&str> for Operands {
    fn from(value: &str) -> Self {
        match value {
            "*" => Self::Mult,
            "+" => Self::Add,
            _ => panic!("Unsupported operand"),
        }
    }
}

#[derive(Debug)]
struct Problem {
    nums: Vec<i64>,
    operand: Operands,
}

fn parse_p1(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = input
        .lines()
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|op| Problem {
            nums: Vec::new(),
            operand: Operands::from(op),
        })
        .collect();

    for line in input.lines().take(input.lines().count() - 1) {
        let nums_in_line: Vec<i64> = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        for (i, num) in nums_in_line.iter().enumerate() {
            problems[i].nums.push(*num);
        }
    }

    problems
}

fn parse_p2(input: &str) -> Vec<Problem> {
    let operands: Vec<Operands> = input
        .lines()
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(Operands::from)
        .collect();

    let original_parse = parse_p1(input);
    let column_widths: Vec<i64> = original_parse
        .iter()
        .map(|problem| {
            problem
                .nums
                .iter()
                .map(|num| num.to_string().len() as i64)
                .max()
                .unwrap()
        })
        .collect();

    // get number for on each column
    let mut column_based_numbers: Vec<i64> = Vec::new();
    let col_size = input.lines().next().unwrap().len();
    let row_size = input.lines().count() - 1;

    for i in 0..col_size {
        let mut current_num: Vec<char> = Vec::new();
        for j in 0..row_size {
            let line = input.lines().nth(j).unwrap();
            let ch = line.chars().nth(i).unwrap();
            if ch != ' ' {
                current_num.push(ch);
            }
        }

        if !current_num.is_empty() {
            let num: i64 = current_num.iter().collect::<String>().parse().unwrap();
            column_based_numbers.push(num);
        }
    }

    // translate column numbers into problems
    let mut cursor = 0;
    let mut problems: Vec<Problem> = Vec::new();
    for (i, operand) in operands.iter().enumerate() {
        let chunk_size = column_widths[i];
        let chunk = column_based_numbers[cursor..cursor + chunk_size as usize].to_vec();
        problems.push(Problem {
            nums: chunk,
            operand: operand.clone(),
        });

        cursor += chunk_size as usize;
    }

    problems
}

fn solve_problem(problem: &Problem) -> i64 {
    let mut sum = problem.nums[0];

    for num in problem.nums.iter().skip(1) {
        match problem.operand {
            Operands::Mult => sum *= num,
            Operands::Add => sum += num,
        }
    }

    sum
}

pub fn solve(input: &str) -> SolutionPair {
    // part 1
    let all_problems = parse_p1(input);
    let sol1: i64 = all_problems.iter().map(solve_problem).sum();

    // part 2
    let all_problems_adjusted = parse_p2(input);
    let sol2: i64 = all_problems_adjusted.iter().map(solve_problem).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day6_p1() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "4277556");
    }
    #[test]
    fn test_example_input_day6_p2() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "3263827");
    }
}
