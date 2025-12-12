advent_of_code::solution!(12);

use good_lp::*;

#[derive(Debug, Clone)]
struct Shape {
    cells: Vec<(i32, i32)>, // occupied cells relative to origin
}

impl Shape {
    fn from_pattern(pattern: &str) -> Self {
        let lines: Vec<&str> = pattern.trim().lines().collect();
        let mut cells = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cells.push((y as i32, x as i32));
                }
            }
        }
        Shape { cells }
    }

    fn rotate_90(&self) -> Self {
        Shape {
            cells: self.cells.iter().map(|(y, x)| (*x, 2 - *y)).collect(),
        }
    }

    fn flip_horizontal(&self) -> Self {
        Shape {
            cells: self.cells.iter().map(|(y, x)| (*y, 2 - *x)).collect(),
        }
    }

    fn get_all_orientations(&self) -> Vec<Shape> {
        let mut orientations = Vec::new();
        let mut seen = std::collections::HashSet::new();

        let mut current = self.clone();
        for _ in 0..4 {
            let normalized = current.normalize();
            let key = format!("{:?}", normalized.cells);
            if seen.insert(key) {
                orientations.push(normalized);
            }
            current = current.rotate_90();
        }

        let mut current = self.flip_horizontal();
        for _ in 0..4 {
            let normalized = current.normalize();
            let key = format!("{:?}", normalized.cells);
            if seen.insert(key) {
                orientations.push(normalized);
            }
            current = current.rotate_90();
        }

        orientations
    }

    fn normalize(&self) -> Self {
        let min_y = self.cells.iter().map(|(y, _)| *y).min().unwrap();
        let min_x = self.cells.iter().map(|(_, x)| *x).min().unwrap();
        Shape {
            cells: self
                .cells
                .iter()
                .map(|(y, x)| (y - min_y, x - min_x))
                .collect(),
        }
    }

    fn height(&self) -> i32 {
        self.cells.iter().map(|(y, _)| *y).max().unwrap() + 1
    }

    fn width(&self) -> i32 {
        self.cells.iter().map(|(_, x)| *x).max().unwrap() + 1
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    fn can_place(&self, shape: &Shape, x: usize, y: usize) -> bool {
        for (dy, dx) in &shape.cells {
            let grid_x = x as i32 + dx;
            let grid_y = y as i32 + dy;

            if grid_x < 0
                || grid_y < 0
                || grid_x >= self.width as i32
                || grid_y >= self.height as i32
            {
                return false;
            }

            let idx = (grid_y as usize) * self.width + (grid_x as usize);
            if self.cells[idx] {
                return false;
            }
        }
        true
    }

    fn place(&mut self, shape: &Shape, x: usize, y: usize) {
        for (dy, dx) in &shape.cells {
            let grid_x = (x as i32 + dx) as usize;
            let grid_y = (y as i32 + dy) as usize;
            let idx = grid_y * self.width + grid_x;
            self.cells[idx] = true;
        }
    }

    fn remove(&mut self, shape: &Shape, x: usize, y: usize) {
        for (dy, dx) in &shape.cells {
            let grid_x = (x as i32 + dx) as usize;
            let grid_y = (y as i32 + dy) as usize;
            let idx = grid_y * self.width + grid_x;
            self.cells[idx] = false;
        }
    }

    fn occupied_count(&self) -> usize {
        self.cells.iter().filter(|&&c| c).count()
    }

    // Count "dead" empty cells - cells in 2x2 regions completely surrounded by occupied cells
    // These can never be filled since all shapes need 3x3 space
    fn count_dead_cells(&self) -> usize {
        let mut dead_cells = vec![false; self.cells.len()];
        let mut count = 0;

        // Check each possible 2x2 region
        for y in 0..self.height.saturating_sub(1) {
            for x in 0..self.width.saturating_sub(1) {
                // Check if this 2x2 is completely surrounded (12 border cells)
                let mut surrounded = true;

                // Check border cells around the 2x2 region
                // Border is from (x-1, y-1) to (x+2, y+2), excluding the 2x2 itself
                for by in (y.saturating_sub(1))..=(y + 2).min(self.height - 1) {
                    for bx in (x.saturating_sub(1))..=(x + 2).min(self.width - 1) {
                        // Skip cells inside the 2x2
                        if by >= y && by < y + 2 && bx >= x && bx < x + 2 {
                            continue;
                        }

                        let idx = by * self.width + bx;
                        if !self.cells[idx] {
                            surrounded = false;
                            break;
                        }
                    }
                    if !surrounded {
                        break;
                    }
                }

                // If surrounded, mark empty cells in this 2x2 as dead
                if surrounded {
                    for dy in 0..2 {
                        for dx in 0..2 {
                            let cy = y + dy;
                            let cx = x + dx;
                            let idx = cy * self.width + cx;

                            if !self.cells[idx] && !dead_cells[idx] {
                                dead_cells[idx] = true;
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        count
    }

    // Find the first empty cell (reading left-to-right, top-to-bottom)
    // Returns None if grid is full
    fn first_empty(&self) -> Option<(usize, usize)> {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if !self.cells[idx] {
                    return Some((x, y));
                }
            }
        }
        None
    }

    // Check if we can greedily place num_shapes 3x3 solid blocks
    // If yes, we definitely have a solution since all shapes fit in 3x3
    fn can_fit_3x3_blocks(&self, num_blocks: usize) -> bool {
        let mut temp = self.cells.clone();
        let mut placed = 0;

        for y in 0..self.height.saturating_sub(2) {
            for x in 0..self.width.saturating_sub(2) {
                // Check if we can place a 3x3 block here
                let mut all_empty = true;
                for dy in 0..3 {
                    for dx in 0..3 {
                        let idx = (y + dy) * self.width + (x + dx);
                        if temp[idx] {
                            all_empty = false;
                            break;
                        }
                    }
                    if !all_empty {
                        break;
                    }
                }

                if all_empty {
                    // Mark this 3x3 region as occupied
                    for dy in 0..3 {
                        for dx in 0..3 {
                            let idx = (y + dy) * self.width + (x + dx);
                            temp[idx] = true;
                        }
                    }
                    placed += 1;
                    if placed >= num_blocks {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn solve_backtracking_recursive(
    grid: &mut Grid,
    shapes_with_orientations: &[Vec<Shape>],
    remaining: &mut Vec<(usize, usize)>, // (shape_idx, remaining_count)
) -> bool {
    // Base case: all shapes placed
    if remaining.is_empty() {
        return true;
    }

    // Early impossibility check: not enough space
    let total_cells_needed: usize = remaining
        .iter()
        .map(|(shape_idx, count)| shapes_with_orientations[*shape_idx][0].cells.len() * count)
        .sum();

    let occupied = grid.occupied_count();
    let dead = grid.count_dead_cells();
    let available_space = grid.width * grid.height - occupied - dead;

    if total_cells_needed > available_space {
        return false;
    }

    // Early success check: if we can fit all remaining shapes as 3x3 blocks,
    // we definitely have a solution (since all shapes fit within 3x3)
    let num_shapes_remaining: usize = remaining.iter().map(|(_, count)| count).sum();
    if grid.can_fit_3x3_blocks(num_shapes_remaining) {
        return true;
    }

    // Pop one shape type to place
    let (shape_idx, count) = remaining.pop().unwrap();

    // Try all orientations and positions
    for orientation in &shapes_with_orientations[shape_idx] {
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.can_place(orientation, x, y) {
                    grid.place(orientation, x, y);

                    // If we need more of this shape, push it back
                    if count > 1 {
                        remaining.push((shape_idx, count - 1));
                    }

                    // Recurse
                    if solve_backtracking_recursive(grid, shapes_with_orientations, remaining) {
                        return true;
                    }

                    // Backtrack
                    if count > 1 {
                        remaining.pop();
                    }

                    grid.remove(orientation, x, y);
                }
            }
        }
    }

    // Restore and fail
    remaining.push((shape_idx, count));
    false
}

fn solve_with_constraint_solver(
    grid_w: usize,
    grid_h: usize,
    shapes: &[Shape],
    counts: &[usize],
) -> bool {
    // Precompute all orientations
    let all_orientations: Vec<Vec<Shape>> =
        shapes.iter().map(|s| s.get_all_orientations()).collect();

    let mut problem = ProblemVariables::new();
    let mut placements = Vec::new();

    // Create variables for each possible placement
    for (shape_idx, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            let mut instance_vars = Vec::new();

            for y in 0..grid_h {
                for x in 0..grid_w {
                    for (_orient_idx, orientation) in all_orientations[shape_idx].iter().enumerate() {
                        // Check if this placement is within bounds
                        let mut valid = true;
                        for (dy, dx) in &orientation.cells {
                            let grid_x = x as i32 + dx;
                            let grid_y = y as i32 + dy;
                            if grid_x < 0 || grid_y < 0 || grid_x >= grid_w as i32 || grid_y >= grid_h as i32 {
                                valid = false;
                                break;
                            }
                        }

                        if valid {
                            let var = problem.add(variable().binary());
                            instance_vars.push((var, x, y, orientation.clone()));
                        }
                    }
                }
            }

            placements.push(instance_vars);
        }
    }

    // Build the model
    let mut model = problem.minimise(0).using(default_solver);

    // Constraint: each shape instance must be placed exactly once
    for instance_vars in &placements {
        let sum: Expression = instance_vars.iter().map(|(var, _, _, _)| *var).sum();
        model = model.with(constraint!(sum == 1));
    }

    // Constraint: each grid cell occupied by at most one shape
    for grid_y in 0..grid_h {
        for grid_x in 0..grid_w {
            let mut occupying_vars = Vec::new();

            for instance_vars in &placements {
                for (var, x, y, orientation) in instance_vars {
                    // Check if this placement covers cell (grid_x, grid_y)
                    for (dy, dx) in &orientation.cells {
                        if (*y as i32 + dy) == grid_y as i32 && (*x as i32 + dx) == grid_x as i32 {
                            occupying_vars.push(*var);
                            break;
                        }
                    }
                }
            }

            if !occupying_vars.is_empty() {
                let sum: Expression = occupying_vars.into_iter().sum();
                model = model.with(constraint!(sum <= 1));
            }
        }
    }

    // Solve
    matches!(model.solve(), Ok(_))
}

#[allow(dead_code)]
fn solve_with_backtracking(
    grid_w: usize,
    grid_h: usize,
    shapes: &[Shape],
    counts: &[usize],
) -> bool {
    // Precompute all orientations
    let shapes_with_orientations: Vec<Vec<Shape>> =
        shapes.iter().map(|s| s.get_all_orientations()).collect();

    // Build list of shapes to place: (shape_idx, count)
    let mut remaining: Vec<(usize, usize)> = counts
        .iter()
        .enumerate()
        .filter(|(_, count)| **count > 0)
        .map(|(idx, count)| (idx, *count))
        .collect();

    // Sort by shape size (descending) - larger shapes first
    // This is a good heuristic as they're more constrained
    remaining.sort_by_key(|(shape_idx, _count)| {
        -(shapes_with_orientations[*shape_idx][0].cells.len() as i32)
    });

    let mut grid = Grid::new(grid_w, grid_h);
    solve_backtracking_recursive(&mut grid, &shapes_with_orientations, &mut remaining)
}

#[derive(Debug, Clone)]
struct Problem {
    i: usize,
    j: usize,
    shape_counts: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Problem>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    // We should have 6 shapes and a list of problems
    let shapes = sections[..6]
        .iter()
        .map(|s| Shape::from_pattern(s))
        .collect::<Vec<_>>();
    let problems = sections[6]
        .lines()
        .enumerate()
        .map(|(_i, line)| {
            let (dim_str, counts_str) = line.split_once(": ").unwrap();
            let (i_dim, j_dim) = dim_str.split_once('x').unwrap();
            let counts = counts_str
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Problem {
                i: usize::from_str_radix(i_dim, 10).unwrap(),
                j: usize::from_str_radix(j_dim, 10).unwrap(),
                shape_counts: counts,
            }
        })
        .collect::<Vec<_>>();
    (shapes, problems)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (shapes, problems) = parse_input(input);

    let count = problems
        .iter()
        .filter(|problem| solve_with_constraint_solver(problem.j, problem.i, &shapes, &problem.shape_counts))
        .count();

    count.try_into().ok()
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
