use std::collections::HashMap;

use petgraph::graph;
use good_lp::*;

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
    // pretty print lights in this format: "[..#..#..]"
    // Where # is on and . is off
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

    // For debug, we will use pretty_print to show the state of the lights
    fn debug(&self) {
        println!("{}", self.pretty_print());
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

    fn pretty_print(&self) -> String {
        let mut result = String::from("(");
        for (i, &toggle) in self.toggles.iter().enumerate() {
            result.push_str(&toggle.to_string());
            if i < self.toggles.len() - 1 {
                result.push(',');
            }
        }
        result.push(')');
        result
    }

    fn debug(&self) {
        println!("{}", self.pretty_print());
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
    fn pretty_print(&self) -> String {
        let mut result = String::from("{");
        for (i, &value) in self.values.iter().enumerate() {
            result.push_str(&value.to_string());
            if i < self.values.len() - 1 {
                result.push(',');
            }
        }
        result.push('}');
        result
    }
    fn debug(&self) {
        println!("{}", self.pretty_print());
    }

    fn apply_schematic(&mut self, schematic: &Schematic) {
        for &index in &schematic.toggles {
            if let Some(value) = self.values.get_mut(index) {
                *value += 1;
            }
        }
    }

    // If any individual joltage is greater than the bound, return false
    fn in_bounds(&self, bound: &Joltages) -> bool {
        for (i, &value) in self.values.iter().enumerate() {
            if let Some(&bound_value) = bound.values.get(i) {
                if value > bound_value {
                    return false;
                }
            }
        }
        true
    }
}

// Improved heuristic that considers the best possible schematic coverage
// We compute: sum of all deficits / size of largest schematic
// This is admissible (optimistic) because it assumes we can always use the largest schematic
fn heuristic(current: &Joltages, target: &Joltages, max_schematic_size: usize) -> u64 {
    let total_deficit: u64 = current
        .values
        .iter()
        .zip(target.values.iter())
        .map(|(&c, &t)| if t > c { t - c } else { 0 })
        .sum();

    // Divide by max schematic size (rounding up)
    if max_schematic_size == 0 {
        return total_deficit;
    }
    (total_deficit + max_schematic_size as u64 - 1) / max_schematic_size as u64
}

// Score a schematic based on how helpful it is for the current state
fn score_schematic(
    schematic: &Schematic,
    current: &[u64],
    target: &[u64],
) -> Option<u64> {
    let mut score = 0u64;

    for &toggle_idx in &schematic.toggles {
        if toggle_idx < current.len() {
            let curr_val = current[toggle_idx];
            let target_val = target[toggle_idx];

            if curr_val < target_val {
                // Weight by deficit - helps joltages that need it most
                score += (target_val - curr_val);
            } else if curr_val >= target_val {
                // Would exceed target - invalid move
                return None;
            }
        }
    }

    Some(score)
}

// Backtracking search with greedy heuristic ordering
fn backtracking_search(
    current: &mut Vec<u64>,
    target: &[u64],
    schematics: &[Schematic],
    depth: u64,
    best_so_far: &mut u64,
    memo: &mut HashMap<Vec<u64>, Option<u64>>,
) -> Option<u64> {
    // Check if we've reached the target
    if current == target {
        return Some(depth);
    }

    // Prune if we've already exceeded the best solution found
    if depth >= *best_so_far {
        return None;
    }

    // Check memo
    if let Some(&cached) = memo.get(current) {
        return cached.map(|c| c + depth);
    }

    // Safety limit to prevent stack overflow
    if depth > 200 {
        return None;
    }

    // Get all valid schematics with their scores
    let mut options: Vec<(usize, u64)> = schematics
        .iter()
        .enumerate()
        .filter_map(|(idx, schematic)| {
            score_schematic(schematic, current, target).map(|score| (idx, score))
        })
        .collect();

    // Sort by score (highest first) - this is the greedy heuristic
    options.sort_by_key(|&(_, score)| std::cmp::Reverse(score));

    // If no valid moves, we're stuck
    if options.is_empty() {
        memo.insert(current.clone(), None);
        return None;
    }

    // Try each option in order (best first, with backtracking)
    for (idx, _score) in options {
        // Apply the schematic
        for &toggle_idx in &schematics[idx].toggles {
            if toggle_idx < current.len() {
                current[toggle_idx] += 1;
            }
        }

        // Recursively search
        if let Some(result) = backtracking_search(current, target, schematics, depth + 1, best_so_far, memo) {
            // Found a solution! Update best
            if result < *best_so_far {
                *best_so_far = result;
            }

            // Undo the move before returning
            for &toggle_idx in &schematics[idx].toggles {
                if toggle_idx < current.len() {
                    current[toggle_idx] -= 1;
                }
            }

            return Some(result);
        }

        // Backtrack - undo the move
        for &toggle_idx in &schematics[idx].toggles {
            if toggle_idx < current.len() {
                current[toggle_idx] -= 1;
            }
        }
    }

    // No solution found from this state
    memo.insert(current.clone(), None);
    None
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

// Main solver - try ILP first, fall back to backtracking for small inputs
fn greedy_solution(
    target_joltages: &Joltages,
    schematics: &[Schematic],
) -> u64 {
    // Try ILP solution
    ilp_solution(target_joltages, schematics)
}

// Validate a solution by simulating the presses
fn validate_solution(presses: &[(usize, u64)], target: &[u64], schematics: &[Schematic]) -> bool {
    let mut current = vec![0u64; target.len()];
    for &(schematic_idx, count) in presses {
        for _ in 0..count {
            for &toggle_idx in &schematics[schematic_idx].toggles {
                if toggle_idx < current.len() {
                    current[toggle_idx] += 1;
                }
            }
        }
    }
    current == target
}

// Main function: try greedy approach
fn lowest_button_count_for_valid_joltages(
    _current_joltages: Joltages,
    target_joltages: &Joltages,
    schematics: &[Schematic],
    _current_count: u64,
    lowest_count: &mut u64,
) -> u64 {
    let result = greedy_solution(target_joltages, schematics);
    *lowest_count = result;
    result
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
            let mut schematics = line
                .split(" ")
                .skip(1)
                .filter_map(|s| Schematic::from_str(s))
                .collect::<Vec<Schematic>>();

            // Sort schematics by size (largest first) - helps A* explore better paths first
            schematics.sort_by_key(|s| std::cmp::Reverse(s.toggles.len()));

            let mut lowest_possible = u64::MAX; // Start with no limit
            let joltage_button_count = lowest_button_count_for_valid_joltages(
                Joltages {
                    values: vec![0; joltages.values.len()],
                },
                &joltages,
                &schematics,
                0,
                &mut lowest_possible,
            );
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
    fn test_lowest_button_count() {
        let current_joltages = Joltages { values: vec![0, 0] };
        let target_joltages = Joltages { values: vec![1, 2] };
        let schematics = vec![
            Schematic { toggles: vec![1] },
            Schematic {
                toggles: vec![0, 1],
            },
        ];
        let mut lowest = u64::MAX;
        let count = lowest_button_count_for_valid_joltages(
            current_joltages,
            &target_joltages,
            &schematics,
            0,
            &mut lowest,
        );
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
        let mut schematics = line
            .split(" ")
            .skip(1)
            .filter_map(|s| Schematic::from_str(s))
            .collect::<Vec<Schematic>>();

        // Sort schematics by size (largest first)
        schematics.sort_by_key(|s| std::cmp::Reverse(s.toggles.len()));

        let mut lowest = u64::MAX;
        let start = std::time::Instant::now();
        let count = lowest_button_count_for_valid_joltages(
            Joltages {
                values: vec![0; joltages.values.len()],
            },
            &joltages,
            &schematics,
            0,
            &mut lowest,
        );
        let elapsed = start.elapsed();

        println!("Found solution: {} in {:?}", count, elapsed);
        // We don't know the expected answer yet, just verify it completes in reasonable time
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
