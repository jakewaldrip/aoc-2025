use crate::{
    Solution, SolutionPair,
    utils::grid::{Grid, Point},
};

///////////////////////////////////////////////////////////////////////////////

fn is_roll_accessible(point: &Point, grid: &Grid) -> bool {
    let num_rolls_as_neighbors: i32 = grid.count_neighbors_of_value(point, b'@');
    num_rolls_as_neighbors < 4
}

pub fn solve(input: &str) -> SolutionPair {
    let mut grid = Grid::new(input);
    let sol1: i32 = grid
        .iter_2d()
        .filter_map(|point| {
            if point.value == b'.' {
                return None;
            }

            if is_roll_accessible(&point, &grid) {
                return Some(1);
            }

            None
        })
        .sum();

    let mut sol2 = 0;
    loop {
        let mut accessible_points: Vec<Point> = Vec::new();
        for point in grid.iter_2d() {
            if point.value == b'@' && is_roll_accessible(&point, &grid) {
                accessible_points.push(point);
            }
        }

        if accessible_points.is_empty() {
            break;
        }

        for point in accessible_points {
            unsafe {
                let ptr = grid
                    .data
                    .as_mut_ptr()
                    .add(point.row * grid.row_size + point.col);
                *ptr = b'.';
                sol2 += 1;
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day4_p1() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "13");
    }

    #[test]
    fn test_example_input_day4_p2() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "43");
    }
}
