use std::collections::HashSet;

advent_of_code::solution!(8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

// Euclidean distance between two junction boxes
fn distance(a: &JunctionBox, b: &JunctionBox) -> f64 {
    let dx = (a.x as i64 - b.x as i64) as f64;
    let dy = (a.y as i64 - b.y as i64) as f64;
    let dz = (a.z as i64 - b.z as i64) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<u64> = line
                .split(',')
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            JunctionBox {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect::<Vec<JunctionBox>>()
}

fn get_potential_connections(junction_boxes: &Vec<JunctionBox>) -> Vec<(usize, usize, f64)> {
    let mut potential_connections: Vec<(usize, usize, f64)> = vec![];
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            let distance = distance(&junction_boxes[i], &junction_boxes[j]);
            potential_connections.push((i, j, distance));
        }
    }
    potential_connections.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    potential_connections
}

fn make_connection(a: &JunctionBox, b: &JunctionBox, circuits: &mut Vec<HashSet<JunctionBox>>) {
    let a_circuit_idx = circuits.iter().position(|circuit| circuit.contains(a));
    let b_circuit_idx = circuits.iter().position(|circuit| circuit.contains(b));

    match (a_circuit_idx, b_circuit_idx) {
        (Some(ac_idx), Some(bc_idx)) => {
            if ac_idx != bc_idx {
                // Need to merge circuits at different indices
                // Clone the one we're merging from
                let to_merge = circuits[bc_idx].clone();
                circuits[ac_idx].extend(to_merge);
                circuits.remove(bc_idx);
            }
        }
        (Some(ac_idx), None) => {
            circuits[ac_idx].insert(b.clone());
        }
        (None, Some(bc_idx)) => {
            circuits[bc_idx].insert(a.clone());
        }
        (None, None) => {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(a.clone());
            new_circuit.insert(b.clone());
            circuits.push(new_circuit);
        }
    }
}

fn connect_closest_n(input: &str, number_to_connect: usize) -> u64 {
    let junction_boxes = parse_input(input);
    let potential_connections = get_potential_connections(&junction_boxes);

    let mut circuits: Vec<HashSet<JunctionBox>> = vec![];
    for i in 0..number_to_connect {
        if circuits.len() >= number_to_connect {
            break;
        }
        let (a, b, _distance) = &potential_connections[i];
        make_connection(&junction_boxes[*a], &junction_boxes[*b], &mut circuits);
    }

    // Now multiply together the sizes of the biggest 3 circuits
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    circuits
        .iter()
        .map(|circuit| circuit.len() as u64)
        .take(3)
        .product()
}

fn connect_until_one_circuit(input: &str) -> u64 {
    let junction_boxes = parse_input(input);
    let potential_connections = get_potential_connections(&junction_boxes);

    let mut circuits: Vec<HashSet<JunctionBox>> = vec![];
    for (a, b, _distance) in potential_connections.iter() {
        make_connection(&junction_boxes[*a], &junction_boxes[*b], &mut circuits);

        // If all of the junction boxes are now connected, break
        if circuits.len() == 1 && circuits[0].len() == junction_boxes.len() {
            // The number we want to return is the x coordinates of the last two connected
            // junctions multiplied together
            return junction_boxes[*a].x * junction_boxes[*b].x;
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(connect_closest_n(input, 1000))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(connect_until_one_circuit(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = connect_closest_n(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
