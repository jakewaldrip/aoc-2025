use crate::{Solution, SolutionPair, utils::grid::Grid};

///////////////////////////////////////////////////////////////////////////////

fn solve_quantum_manifolds(grid: &Grid) -> (i32, i128) {
    let mut splits: Vec<i128> = vec![0; grid.row_size];
    let mut sum = 0;
    for row in grid.iter_rows() {
        for (i, ch) in row.iter().enumerate() {
            match ch {
                b'S' => splits[i] = 1,
                b'.' => continue,
                b'^' => {
                    let current_val_at_split = splits[i];
                    splits[i] = 0;
                    splits[i - 1] += current_val_at_split;
                    splits[i + 1] += current_val_at_split;

                    if current_val_at_split > 0 {
                        sum += 1;
                    }
                }
                _ => panic!("Bad input file value"),
            }
        }
    }

    (sum, splits.iter().sum::<i128>())
}

pub fn solve(input: &str) -> SolutionPair {
    let grid = Grid::new(input);
    let (sol1, sol2) = solve_quantum_manifolds(&grid);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day7_p1() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "21");
    }

    #[test]
    fn test_example_input_day7_p2() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "40");
    }
}
