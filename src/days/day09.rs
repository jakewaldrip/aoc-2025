use std::cmp::{max, min};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let num_strs = value.split_once(',').unwrap();
        Self {
            x: num_strs.0.parse().unwrap(),
            y: num_strs.1.parse().unwrap(),
        }
    }
}

fn get_area(p1: &Point, p2: &Point) -> i64 {
    let width = (p1.x - p2.x + 1).abs();
    let height = (p1.y - p2.y + 1).abs();
    width * height
}

fn calculate_all_areas(points: &[Point]) -> Vec<(Point, Point, i64)> {
    let mut all_areas: Vec<(Point, Point, i64)> = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            let area = get_area(p1, p2);
            all_areas.push((*p1, *p2, area));
        }
    }

    all_areas
}

fn is_valid_rectangle(candidate: &(Point, Point, i64), edges: &[(Point, Point)]) -> bool {
    let min_y = min(candidate.0.y, candidate.1.y);
    let max_y = max(candidate.0.y, candidate.1.y);
    let min_x = min(candidate.0.x, candidate.1.x);
    let max_x = max(candidate.0.x, candidate.1.x);

    for (p1, p2) in edges {
        let e_min_y = min(p1.y, p2.y);
        let e_max_y = max(p1.y, p2.y);
        let e_min_x = min(p1.x, p2.x);
        let e_max_x = max(p1.x, p2.x);

        // reject if any edge overlaps the rectangle interior
        if min_y < e_max_y && max_y > e_min_y && min_x < e_max_x && max_x > e_min_x {
            return false;
        }
    }
    true
}

fn get_largest_rectange_p1(points: &[Point]) -> i64 {
    let all_areas = calculate_all_areas(points);
    all_areas
        .iter()
        .map(|candidates| candidates.2)
        .max()
        .unwrap()
}

fn get_largest_rectangle_p2(candidates: &[(Point, Point, i64)], points: &[(Point, Point)]) -> i64 {
    candidates
        .iter()
        .find(|candidate| is_valid_rectangle(candidate, points))
        .map(|candidate| candidate.2)
        .unwrap_or(0)
}

pub fn solve(input: &str) -> SolutionPair {
    let red_tiles: Vec<Point> = input.lines().map(Point::from).collect();
    let sol1 = get_largest_rectange_p1(&red_tiles);

    // Get all possible
    let mut all_areas = calculate_all_areas(&red_tiles);
    all_areas.sort_by_key(|a| -a.2);

    // Get all edges of the polygon
    let mut edges: Vec<(Point, Point)> = red_tiles.windows(2).map(|w| (w[0], w[1])).collect();
    edges.push((red_tiles[red_tiles.len() - 1], red_tiles[0]));

    let sol2 = get_largest_rectangle_p2(&all_areas, &edges);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day9_p1() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "50");
    }

    #[test]
    fn test_example_input_day9_p2() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "24");
    }

    #[test]
    fn test_get_area() {
        let p1 = Point { x: 2, y: 5 };
        let p2 = Point { x: 9, y: 7 };
        let result = get_area(&p1, &p2);
        assert_eq!(result, 24);
    }
}
