use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
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
enum Alignment {
    Left,
    Right,
}

#[derive(Debug)]
struct Problem {
    nums: Vec<i64>,
    operand: Operands,
    alignment: Alignment,
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
            // Dont need for p1
            alignment: Alignment::Left,
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
    let mut operands: Vec<Operands> = input
        .lines()
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(Operands::from)
        .collect();

    // need to figure out alignment of column as I encounter it, replace extra spaces with 0's,
    // then parse into numbers and save that number into the vec of problems
    //
    // possibly parse with p1, get largest num of digits in line, this tells you max size of column
    // then find the first number less than that
    // then find that number and check something
    todo!()
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
    // let sol2: i64 = all_problems_adjusted.iter().map(solve_problem).sum();
    let sol2 = 0;

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
        assert_eq!(p2_result, "0");
    }
}
