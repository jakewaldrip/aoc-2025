use std::{collections::HashMap, iter::repeat_n};

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

fn build_circuits_p1(junctions: &[Vector3], distances: &[(&KeyPair, &i64)]) -> i32 {
    let mut circuits: Vec<Vec<Vector3>> = Vec::new();
    let mut vec_to_circuit: HashMap<Vector3, usize> = HashMap::new();

    for (keypair, _) in distances.iter().take(1000) {
        let i1 = vec_to_circuit.get(&keypair.0).copied();
        let i2 = vec_to_circuit.get(&keypair.1).copied();

        match (i1, i2) {
            // same circuit already, skip
            (Some(idx1), Some(idx2)) if idx1 == idx2 => continue,

            // separate circuits, merge
            (Some(idx1), Some(idx2)) => {
                let (keep, remove) = if idx1 < idx2 {
                    (idx1, idx2)
                } else {
                    (idx2, idx1)
                };
                let mut removed = circuits.swap_remove(remove);
                circuits[keep].append(&mut removed);

                // update hashmap for the merged circuit
                for v in &circuits[keep] {
                    vec_to_circuit.insert(*v, keep);
                }
                // update the circuit that was moved to remove index
                if remove < circuits.len() {
                    for v in &circuits[remove] {
                        vec_to_circuit.insert(*v, remove);
                    }
                }
            }

            // v1 has a circuit, add v2
            (Some(idx), None) => {
                circuits[idx].push(keypair.1);
                vec_to_circuit.insert(keypair.1, idx);
            }

            // v2 has a circuit, add v1
            (None, Some(idx)) => {
                circuits[idx].push(keypair.0);
                vec_to_circuit.insert(keypair.0, idx);
            }

            // new circuit with v1 and v2
            (None, None) => {
                let idx = circuits.len();
                circuits.push(vec![keypair.0, keypair.1]);
                vec_to_circuit.insert(keypair.0, idx);
                vec_to_circuit.insert(keypair.1, idx);
            }
        }
    }

    let total_connected = circuits.iter().map(|c| c.len()).sum::<usize>();
    let num_singles = junctions.len() - total_connected;
    let mut sizes: Vec<i32> = circuits.iter().map(|c| c.len() as i32).collect();
    sizes.extend(repeat_n(1, num_singles));

    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

pub fn solve(input: &str) -> SolutionPair {
    let junctions: Vec<Vector3> = input.lines().map(Vector3::from).collect();
    let distance_map = create_distance_map(&junctions);
    let mut distances: Vec<_> = distance_map.iter().collect();
    distances.sort_by_key(|&(_, v)| v);

    // part 1
    let sol1 = build_circuits_p1(&junctions, &distances);

    // part 2
    let sol2 = 0;
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
