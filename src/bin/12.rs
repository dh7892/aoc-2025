advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Shape {
    cells: Vec<(i32, i32)>,
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
}

#[derive(Debug)]
struct Problem {
    i: usize,
    j: usize,
    shape_counts: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Problem>) {
    let lines: Vec<&str> = input.lines().collect();

    // Parse shapes - find lines 0-5 and their patterns
    let mut shapes = Vec::new();
    let mut i = 0;

    while i < lines.len() && shapes.len() < 6 {
        let line = lines[i].trim();
        if line.ends_with(':') && line.len() == 2 {
            // Found a shape marker, collect the 3 lines of pattern
            let mut pattern_lines = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].contains(':') && !lines[i].is_empty() {
                pattern_lines.push(lines[i]);
                i += 1;
            }
            if pattern_lines.len() > 0 {
                shapes.push(Shape::from_pattern(&pattern_lines.join("\n")));
            }
        } else {
            i += 1;
        }
    }

    // Parse problems - lines with format "WxH: n0 n1 n2 n3 n4 n5"
    let problems = lines
        .iter()
        .filter(|line| line.contains('x') && line.contains(':'))
        .map(|line| {
            let line = line.trim();
            let (dim_str, counts_str) = line.split_once(": ").unwrap();
            let (w_str, h_str) = dim_str.split_once('x').unwrap();
            let counts = counts_str
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Problem {
                i: w_str.parse().unwrap(),
                j: h_str.parse().unwrap(),
                shape_counts: counts,
            }
        })
        .collect::<Vec<_>>();

    (shapes, problems)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (shapes, problems) = parse_input(input);

    // Calculate actual cell count for each shape
    let shape_cells: Vec<usize> = shapes.iter().map(|s| s.cells.len()).collect();

    let count = problems
        .iter()
        .filter(|problem| {
            let area = problem.i * problem.j;

            // Calculate total cells needed based on actual shape sizes
            let total_cells_needed: usize = problem.shape_counts
                .iter()
                .enumerate()
                .map(|(idx, &count)| count * shape_cells[idx])
                .sum();

            // Problem is solvable if cells needed <= area
            total_cells_needed <= area
        })
        .count();

    count.try_into().ok()
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}
