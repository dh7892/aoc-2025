use std::collections::HashMap;

use good_lp::*;
use petgraph::graph;

advent_of_code::solution!(10);

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Lights {
    state: Vec<bool>,
}

impl Lights {
    fn from_length(length: usize) -> Self {
        Self {
            state: vec![false; length],
        }
    }

    fn new() -> Self {
        Self { state: Vec::new() }
    }
    // We look for a substring in the form "[..#..#..]" where # is on and . is off
    // and parse it into a Lights struct
    // Returns None if the string is not in the correct format
    fn from_str(s: &str) -> Option<Self> {
        let mut lights = Self::new();
        let mut found_start = false;
        for c in s.chars() {
            if c == '[' {
                found_start = true;
                continue;
            }
            if c == ']' {
                break;
            }
            if found_start {
                match c {
                    '#' => lights.append(true),
                    '.' => lights.append(false),
                    _ => return None,
                }
            }
        }
        Some(lights)
    }
    fn append(&mut self, on: bool) {
        self.state.push(on);
    }
    fn toggle(&mut self, indices: &[usize]) {
        for &i in indices {
            if let Some(light) = self.state.get_mut(i) {
                *light = !*light;
            }
        }
    }

    // Pretty print lights in this format: "[..#..#..]"
    // Where # is on and . is off (used in tests)
    #[cfg(test)]
    fn pretty_print(&self) -> String {
        let mut result = String::from("[");
        for &light in &self.state {
            if light {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push(']');
        result
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Schematic {
    toggles: Vec<usize>,
}

impl Schematic {
    // If our string is in the format "(1,2,3)" then parse it into a Schematic
    // Otherwise return None
    fn from_str(s: &str) -> Option<Self> {
        let s = s.trim();
        if !s.starts_with('(') || !s.ends_with(')') {
            return None;
        }
        let s = &s[1..s.len() - 1];
        let toggles: Vec<usize> = s
            .split(',')
            .filter_map(|part| part.trim().parse::<usize>().ok())
            .collect();

        Some(Self { toggles })
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Joltages {
    values: Vec<u64>,
}

impl Joltages {
    // Look for a substring of the form "{1,2,3}" and parse it into a Joltages struct
    // Returns None if the string is not in the correct format
    fn from_str(s: &str) -> Option<Self> {
        // Find the substring between { and }
        let start = s.find('{')?;
        let end = s.find('}')?;
        let inner = &s[start + 1..end];

        // Split by comma and parse each number
        let values: Vec<u64> = inner
            .split(',')
            .filter_map(|part| part.trim().parse::<u64>().ok())
            .collect();

        Some(Self { values })
    }
}

// ILP solution: Solve the integer linear program
// Variables: x_i = number of presses for schematic i
// Objective: minimize sum(x_i)
// Constraints: For each joltage j, sum(x_i where schematic i affects j) = target_j
fn ilp_solution(
    target_joltages: &Joltages,
    schematics: &[Schematic],
) -> u64 {
    let num_schematics = schematics.len();
    let num_joltages = target_joltages.values.len();

    // Create the problem
    let mut problem = ProblemVariables::new();

    // Create variables for each schematic (number of times to press)
    let press_counts: Vec<Variable> = (0..num_schematics)
        .map(|i| problem.add(variable().name(format!("x{}", i)).integer().min(0)))
        .collect();

    // Build the objective: minimize sum of all press counts
    let objective = press_counts.iter().sum::<Expression>();

    // Create the optimization problem with objective
    let mut solver = problem
        .minimise(objective)
        .using(default_solver);

    // Add constraints for each joltage
    for joltage_idx in 0..num_joltages {
        let target = target_joltages.values[joltage_idx] as i32;

        // Find all schematics that affect this joltage
        let mut expr = Expression::from(0);
        for (schematic_idx, schematic) in schematics.iter().enumerate() {
            if schematic.toggles.contains(&joltage_idx) {
                expr = expr + press_counts[schematic_idx];
            }
        }

        // Constraint: sum of presses for schematics affecting this joltage = target
        solver = solver.with(constraint!(expr == target));
    }

    // Solve the problem
    let solution = solver.solve();

    match solution {
        Ok(sol) => {
            // Sum up the total number of presses
            let total: u64 = press_counts
                .iter()
                .map(|&var| sol.value(var).round() as u64)
                .sum();

            total
        }
        Err(_e) => {
            u64::MAX
        }
    }
}

// Recurrsively generate all possible light states for a given number of lights
// Each light can be on or off
fn all_light_states(num_lights: usize, states: Vec<Lights>) -> Vec<Lights> {
    if num_lights == 0 {
        return states;
    }
    let mut new_states = Vec::new();
    for state in states {
        let mut state_on = state.clone();
        state_on.append(true);
        new_states.push(state_on);
        let mut state_off = state.clone();
        state_off.append(false);
        new_states.push(state_off);
    }
    all_light_states(num_lights - 1, new_states)
}
fn shortest_path_to_goal(lights: &Lights, schematics: &[Schematic]) -> u64 {
    let num_lights = lights.state.len();
    let all_states = all_light_states(num_lights, vec![Lights::new()]);

    // Create a petgraph graph of our light states
    let mut graph: graph::Graph<Lights, ()> = graph::Graph::new();
    let mut state_indices: HashMap<Lights, graph::NodeIndex> = HashMap::new();
    for state in &all_states {
        let idx = graph.add_node(state.clone());
        state_indices.insert(state.clone(), idx);
    }

    // Now, we loop through all light states , see which state we'd get by applying each schematic,
    // and add an edge to the graph
    for state in &all_states {
        for schematic in schematics {
            let mut new_state = state.clone();
            new_state.toggle(&schematic.toggles);
            if let Some(&new_idx) = state_indices.get(&new_state) {
                let current_idx = state_indices.get(state).unwrap();
                graph.add_edge(*current_idx, new_idx, ());
            }
        }
    }

    let start_lights = Lights::from_length(num_lights);
    let start_idx = state_indices
        .get(&start_lights)
        .expect("Start light state not found in graph");

    let target_idx = state_indices
        .get(lights)
        .expect("Target light state not found in graph");

    let paths = petgraph::algo::dijkstra(&graph, *start_idx, Some(*target_idx), |_| 1);
    *paths.get(target_idx).unwrap_or(&u64::MAX)
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| {
            let lights = Lights::from_str(line).unwrap();
            let schematics = line
                .split(" ")
                .skip(1)
                .filter_map(|s| Schematic::from_str(s))
                .collect::<Vec<Schematic>>();
            shortest_path_to_goal(&lights, &schematics)
        })
        .sum::<u64>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let joltages = Joltages::from_str(line).unwrap();
            let schematics = line
                .split(" ")
                .skip(1)
                .filter_map(|s| Schematic::from_str(s))
                .collect::<Vec<Schematic>>();

            let joltage_button_count = ilp_solution(&joltages, &schematics);

            if joltage_button_count == u64::MAX {
                // No solution found - skip this line
                0
            } else {
                joltage_button_count
            }
        })
        .sum::<u64>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_ilp_solution() {
        let target_joltages = Joltages { values: vec![1, 2] };
        let schematics = vec![
            Schematic { toggles: vec![1] },
            Schematic {
                toggles: vec![0, 1],
            },
        ];
        let count = ilp_solution(&target_joltages, &schematics);
        // To get [1, 2]:
        // Press schematic[1] (0,1) once: [1, 1]
        // Press schematic[0] (1) once: [1, 2]
        // Total: 2 presses
        assert_eq!(count, 2);
    }

    #[test]
    fn test_real_input_line1() {
        let line = "[.#......#.] (2,9) (3,5,6,7,8) (0,7,8,9) (4) (0,2,3) (2,3,4,5,6,7,8,9) (1,2,3,7) (1,8) (0,2,5,6,9) (0,1,2,3,5,6,7) {59,48,81,71,11,42,42,70,42,42}";
        let joltages = Joltages::from_str(line).unwrap();
        let schematics = line
            .split(" ")
            .skip(1)
            .filter_map(|s| Schematic::from_str(s))
            .collect::<Vec<Schematic>>();

        let start = std::time::Instant::now();
        let count = ilp_solution(&joltages, &schematics);
        let elapsed = start.elapsed();

        println!("Found solution: {} in {:?}", count, elapsed);
        // Verify it completes in reasonable time
        assert!(elapsed.as_secs() < 5, "Should complete in under 5 seconds");
        assert_ne!(count, u64::MAX, "Should find a solution");
    }

    #[test]
    fn test_all_light_states() {
        let initial_states = vec![Lights::new()];
        let states = all_light_states(2, initial_states);
        assert_eq!(states.len(), 4);
        assert_eq!(states[3].pretty_print(), "[..]");
        assert_eq!(states[2].pretty_print(), "[.#]");
        assert_eq!(states[1].pretty_print(), "[#.]");
        assert_eq!(states[0].pretty_print(), "[##]");
    }
}
