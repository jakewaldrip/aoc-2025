use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Vector3 {
    fn from(value: &str) -> Self {
        let nums: Vec<i64> = value.split(',').map(|num| num.parse().unwrap()).collect();
        Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct KeyPair(Vector3, Vector3);

impl KeyPair {
    fn new(v1: Vector3, v2: Vector3) -> Self {
        if v1 < v2 { Self(v1, v2) } else { Self(v2, v1) }
    }
}

#[derive(Debug)]
struct UnionFind {
    parent: HashMap<Vector3, Vector3>,
    rank: HashMap<Vector3, usize>,
    size: HashMap<Vector3, usize>,
}

impl UnionFind {
    fn new(junctions: &[Vector3]) -> Self {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();
        let mut size = HashMap::new();

        for &v in junctions {
            parent.insert(v, v);
            rank.insert(v, 0);
            size.insert(v, 1);
        }

        Self { parent, rank, size }
    }

    fn find(&mut self, v: Vector3) -> Vector3 {
        // Different parent, recursively find and compress path
        if self.parent[&v] != v {
            let parent = self.find(self.parent[&v]);
            self.parent.insert(v, parent);
        }

        self.parent[&v]
    }

    fn union(&mut self, v1: Vector3, v2: Vector3) {
        let parent_v1 = self.find(v1);
        let parent_v2 = self.find(v2);

        if parent_v1 == parent_v2 {
        } else if self.rank[&parent_v1] > self.rank[&parent_v2] {
            self.parent.insert(parent_v2, parent_v1);
            *self.size.get_mut(&parent_v1).unwrap() += self.size[&parent_v2];
        } else if self.rank[&parent_v2] > self.rank[&parent_v1] {
            self.parent.insert(parent_v1, parent_v2);
            *self.size.get_mut(&parent_v2).unwrap() += self.size[&parent_v1];
        } else {
            self.parent.insert(parent_v2, parent_v1);
            *self.size.get_mut(&parent_v1).unwrap() += self.size[&parent_v2];
            *self.rank.get_mut(&parent_v1).unwrap() += 1
        }
    }

    fn get_sizes(&self) -> Vec<usize> {
        self.size
            .iter()
            .filter(|&(k, _)| self.parent[k] == *k)
            .map(|(_, &s)| s)
            .collect()
    }
}

fn create_distance_map(junctions: &[Vector3]) -> HashMap<KeyPair, i64> {
    let mut distance_map: HashMap<KeyPair, i64> = HashMap::new();

    for (i, junction) in junctions.iter().enumerate() {
        for (j, target_junction) in junctions.iter().enumerate() {
            // Don't calc the same vector to itself, or the same distance twice
            // optimization opportunity, find a way to not clone here
            let keypair = KeyPair::new(*junction, *target_junction);
            if i == j {
                continue;
            }

            let distance = junction.calc_euc_dist(target_junction);
            distance_map.insert(keypair, distance);
        }
    }

    distance_map
}

impl Vector3 {
    fn calc_euc_dist(&self, target: &Vector3) -> i64 {
        // Don't bother with sqrt here, this gives us enough info
        // we don't need to know the actual distance to sort
        (self.x - target.x).pow(2) + (self.y - target.y).pow(2) + (self.z - target.z).pow(2)
    }
}

// You start with a hash map of single size circuits
// You continue to union the circuits of each vector pair until you get 1000
fn build_circuits_p1(junctions: &[Vector3], distances: &[(&KeyPair, &i64)]) -> i32 {
    let mut union_find = UnionFind::new(junctions);
    for (keypair, _) in distances.iter().take(1000) {
        union_find.union(keypair.0, keypair.1);
    }

    let mut sizes: Vec<i32> = union_find
        .get_sizes()
        .into_iter()
        .map(|s| s as i32)
        .collect();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

// You start with a hashmap of single sized circuits
// You union the circuits until .find tells you you're unioning the last two
// You then capture the value we need from the keypair and break
fn build_circuits_p2(junctions: &[Vector3], distances: &[(&KeyPair, &i64)]) -> i64 {
    // Track number of circuits
    let mut circuits = junctions.len();
    let mut union_find = UnionFind::new(junctions);

    for (keypair, _) in distances.iter() {
        // Find the root for both vectors
        let v1 = union_find.find(keypair.0);
        let v2 = union_find.find(keypair.1);

        // They have different roots, union them
        if v1 != v2 {
            circuits -= 1;
            union_find.union(v1, v2);

            // We've unioned the last pair to make a single circuit
            if circuits == 1 {
                return keypair.0.x * keypair.1.x;
            }
        }
    }

    0
}

pub fn solve(input: &str) -> SolutionPair {
    let junctions: Vec<Vector3> = input.lines().map(Vector3::from).collect();
    let distance_map = create_distance_map(&junctions);
    let mut distances: Vec<_> = distance_map.iter().collect();
    distances.sort_by_key(|&(_, v)| v);

    let sol1 = build_circuits_p1(&junctions, &distances);
    let sol2 = build_circuits_p2(&junctions, &distances);

    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_day8_p1() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
        let (p1, _) = solve(input);
        let p1_result = format!("{p1}");
        assert_eq!(p1_result, "40");
    }

    #[test]
    fn test_example_input_day8_p2() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
        let (_, p2) = solve(input);
        let p2_result = format!("{p2}");
        assert_eq!(p2_result, "25272");
    }
}
