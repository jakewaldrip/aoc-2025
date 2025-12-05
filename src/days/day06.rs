use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    println!("Input: \n{input}");
    let sol1 = 0;
    let sol2 = 0;
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day6_p1() {
        let input = "";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "0");
    }
}
