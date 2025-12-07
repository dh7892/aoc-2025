use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut beam_cols: HashSet<usize> = HashSet::new();
    let mut num_splits = 0;
    for line in input.lines() {
        if beam_cols.is_empty() {
            // Must be first row, so look for char `S` to start our beam
            for (col, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    beam_cols.insert(col);
                }
            }
        } else {
            // If we find a '^' on a column in beam_cols, we have a split
            for (col, ch) in line.chars().enumerate() {
                if ch == '^' && beam_cols.contains(&col) {
                    num_splits += 1;
                    // remove the column from beam_cols and replace it with col-1 and col+1
                    beam_cols.remove(&col);
                    beam_cols.insert(col - 1);
                    beam_cols.insert(col + 1);
                }
            }
        }
    }
    Some(num_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Now, we don't just track beams, we have to count number that might overlap
    let mut beam_cols: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        if beam_cols.is_empty() {
            // Must be first row, so look for char `S` to start our beam
            for (col, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    beam_cols.insert(col, 1);
                }
            }
        } else {
            // If we find a '^' on a column in beam_cols, we have a split
            for (col, ch) in line.chars().enumerate() {
                if ch == '^'
                    && let Some(count) = beam_cols.remove(&col)
                {
                    *beam_cols.entry(col - 1).or_insert(0) += count;
                    *beam_cols.entry(col + 1).or_insert(0) += count;
                }
            }
        }
        // Print out the current beam_cols for debugging
    }
    Some(beam_cols.values().sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
