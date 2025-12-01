use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
struct Sequence {
    dir: Dir,
    num: i32,
}

fn parse(input: &str) -> Vec<Sequence> {
    let mut sequences: Vec<Sequence> = Vec::new();
    for line in input.lines() {
        let (dir_raw, num_raw) = line.split_at(1);

        let dir = match dir_raw {
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("Invalid value"),
        };

        let num: i32 = num_raw.parse().unwrap();

        let sequence = Sequence { dir, num };
        sequences.push(sequence);
    }

    sequences
}

fn solve_p1(sequences: &[Sequence]) -> i32 {
    let mut counter = 0;
    let mut dial: i32 = 50;

    for sequence in sequences {
        dial = apply_sequence_p1(sequence, &dial);

        if dial == 0 {
            counter += 1;
        }
    }

    counter
}

fn apply_sequence_p1(sequence: &Sequence, dial: &i32) -> i32 {
    let sign_mult = match sequence.dir {
        Dir::Left => -1,
        Dir::Right => 1,
    };
    let delta = sign_mult * sequence.num;
    (dial + delta) % 100
}

fn solve_p2(sequences: &[Sequence]) -> i32 {
    let mut counter = 0;
    let mut dial: i32 = 50;

    for sequence in sequences {
        let (new_dial, times_passed) = apply_sequence_p2(sequence, &dial);
        dial = new_dial;
        counter += times_passed;
    }

    counter
}

fn apply_sequence_p2(sequence: &Sequence, dial: &i32) -> (i32, i32) {
    let sign_mult = match sequence.dir {
        Dir::Left => -1,
        Dir::Right => 1,
    };

    // addresses edge case in slight overcounting when counting X / 100, with + 1 for boundary
    // passing
    // 50 -> L50 was counting as 0 passes due to not _crossing_ the boundary
    // but adding that overcounted other cases that landed on zero
    let offset = match sequence.dir {
        Dir::Right => (100 - (dial % 100)) % 100,
        Dir::Left => dial % 100,
    };

    let zero_passes = if offset == 0 {
        // we are at 0
        sequence.num / 100
    } else if offset <= sequence.num {
        // we will be passing the boundary
        ((sequence.num - offset) / 100) + 1
    } else {
        // we will not be passing the boundary
        0
    };

    // % is not modulo in rust, its the raw remainder (which can result in negative values)
    // ie -18 - 100 = -18
    // This results in 82 for the mathmatical modulo
    let new_dial = (dial + sign_mult * sequence.num).rem_euclid(100);
    (new_dial, zero_passes)
}

pub fn solve(input: &str) -> SolutionPair {
    let parsed_input = parse(input);
    let sol1 = solve_p1(&parsed_input).to_string();
    let sol2 = solve_p2(&parsed_input).to_string();

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day1_p1() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "3");
    }

    #[test]
    fn test_example_input_large_num_day1_p1() {
        let input = "L168\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "3");
    }

    #[test]
    fn test_example_input_day1_p2() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "6");
    }

    #[test]
    fn test_example_input_large_num_day1_p2() {
        let input = "L168\nL30\nR448\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "11");
    }

    #[test]
    fn test_example_input_zero_to_zero_day1_p2() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR100\nL100";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "7");
    }

    #[test]
    fn test_left_large_movement_overcounting() {
        let input = "L50";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "1");
    }
}
