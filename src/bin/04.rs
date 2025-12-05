use std::collections::HashSet;

advent_of_code::solution!(4);

// Return the count of adjacent elements in a 2D grid.
// Taking account that edges/corners have less than 8 adjacent elements.
fn get_num_adjacent_elements<'a>(
    grid: &HashSet<(usize, usize)>,
    row: &usize,
    col: &usize,
) -> usize {
    let mut adjacent = 0;
    for r in row.saturating_sub(1)..=row + 1 {
        for c in col.saturating_sub(1)..=col + 1 {
            if (r, c) != (*row, *col) && grid.contains(&(r, c)) {
                adjacent += 1;
            }
        }
    }
    adjacent
}

// Return a copy of our grid with all accessible elements removed
// An accessible element is one that has fewer than 4 adjacent elements
fn remove_accessible_elements(grid: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut new_grid = grid.clone();
    for &(r, c) in grid.iter() {
        if get_num_adjacent_elements(grid, &r, &c) < 4 {
            new_grid.remove(&(r, c));
        }
    }
    new_grid
}

fn recursive_remove_accessible_elements(grid: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let new_grid = remove_accessible_elements(grid);
    if new_grid.len() == grid.len() {
        new_grid
    } else {
        recursive_remove_accessible_elements(&new_grid)
    }
}

fn parse_input(input: &str) -> HashSet<(usize, usize)> {
    let mut grid = HashSet::new();
    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            if ch == '@' {
                grid.insert((row_idx, col_idx));
            }
        }
    }
    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let result = grid
        .iter()
        .filter(|&&(r, c)| get_num_adjacent_elements(&grid, &r, &c) < 4)
        .count();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let final_grid = recursive_remove_accessible_elements(&grid);
    Some((grid.len() - final_grid.len()) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
