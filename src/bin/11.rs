advent_of_code::solution!(11);

use petgraph::algo::{all_simple_paths, toposort};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn build_graph_from_input(input: &str) -> (DiGraph<String, ()>, HashMap<String, NodeIndex>) {
    let mut graph = DiGraph::new();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    for line in input.lines() {
        if let Some((source, targets)) = line.split_once(':') {
            let source = source.trim();
            let source_idx = *node_map
                .entry(source.to_string())
                .or_insert_with(|| graph.add_node(source.to_string()));

            for target in targets.split_whitespace() {
                let target_idx = *node_map
                    .entry(target.to_string())
                    .or_insert_with(|| graph.add_node(target.to_string()));
                graph.add_edge(source_idx, target_idx, ());
            }
        }
    }

    (graph, node_map)
}

fn count_paths_dag(graph: &DiGraph<String, ()>, start: NodeIndex, end: NodeIndex) -> usize {
    // Get topological order
    let topo_order = toposort(graph, None).expect("Graph has cycles!");

    // Initialize counts
    let mut counts: HashMap<NodeIndex, usize> = HashMap::new();
    counts.insert(start, 1);

    // Propagate counts in topological order
    for &node in &topo_order {
        if let Some(&count) = counts.get(&node) {
            for edge in graph.edges(node) {
                let target = edge.target();
                *counts.entry(target).or_insert(0) += count;
            }
        }
    }

    *counts.get(&end).unwrap_or(&0)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (graph, node_map) = build_graph_from_input(input);
    let start = *node_map.get("you")?;
    let end = *node_map.get("out")?;
    let paths: Vec<Vec<NodeIndex>> =
        all_simple_paths::<Vec<NodeIndex>, _, RandomState>(&graph, start, end, 0, Some(50))
            .collect();

    Some(paths.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (graph, node_map) = build_graph_from_input(input);
    let start = *node_map.get("svr")?;
    let end = *node_map.get("out")?;

    let dac = node_map.get("dac")?;
    let fft = node_map.get("fft")?;

    // Find all svr -> dac -> fft -> out paths
    // Then find all svr -> fft -> dac -> out paths
    // and sum their counts

    let segment_1_paths = count_paths_dag(&graph, start, *dac);
    let segment_2_paths = count_paths_dag(&graph, *dac, *fft);
    let segment_3_paths = count_paths_dag(&graph, *fft, end);
    let mut total_paths = segment_1_paths * segment_2_paths * segment_3_paths;

    let segment_1_paths_alt = count_paths_dag(&graph, start, *fft);
    let segment_2_paths_alt = count_paths_dag(&graph, *fft, *dac);
    let segment_3_paths_alt = count_paths_dag(&graph, *dac, end);
    total_paths += segment_1_paths_alt * segment_2_paths_alt * segment_3_paths_alt;

    Some(total_paths as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let part_2_input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        let result = part_two(&part_2_input);
        assert_eq!(result, Some(2));
    }
}
