use std::collections::HashSet;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Corner {
    x: i64,
    y: i64,
}

impl Corner {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    fn area_with_other(&self, other: &Corner) -> u64 {
        let width = (other.x - self.x).abs() as u64 + 1;
        let height = (other.y - self.y).abs() as u64 + 1;
        width * height
    }
}

fn point_inside_polygon(point: &Corner, polygon: &Vec<Corner>) -> bool {
    // Ray-casting algorithm to determine if point is inside polygon
    // Note: we consider points on an edge as inside
    let mut inside = false;
    let n = polygon.len();
    for i in 0..n {
        let j = (i + n - 1) % n;
        let xi = polygon[i].x;
        let yi = polygon[i].y;
        let xj = polygon[j].x;
        let yj = polygon[j].y;

        if ((yi > point.y) != (yj > point.y))
            && (point.x < (xj - xi) * (point.y - yi) / (yj - yi) + xi)
        {
            inside = !inside;
        }

        // Check if point is on the edge
        let min_x = xi.min(xj);
        let max_x = xi.max(xj);
        let min_y = yi.min(yj);
        let max_y = yi.max(yj);
        if (yj - yi) * (point.x - xi) == (xj - xi) * (point.y - yi)
            && point.x >= min_x
            && point.x <= max_x
            && point.y >= min_y
            && point.y <= max_y
        {
            return true; // Point is on the edge
        }
    }

    inside
}

fn rectangle_goes_outside_bounds(c1: &Corner, c2: &Corner, shape: &Vec<Corner>) -> bool {
    // We know the corners given are inside the shape
    // So we only need to check the complimentary corners
    let corner_set: HashSet<Corner> = shape.iter().cloned().collect();
    let corners_to_check = vec![Corner::new(c1.x, c2.y), Corner::new(c2.x, c1.y)];
    for corner in corners_to_check {
        if corner_set.contains(&corner) {
            // If corner is on a corner of the shape, it's definitely inside
            continue;
        }
        // This corner is not on a corner of the shape, so we have to check if it's inside or
        // outside that polygon
        if !point_inside_polygon(&corner, shape) {
            return true;
        }
    }
    // None of the corners of the rectangle went outside the bounds
    // Now we have to check and corners of the shape that might be inside the rectangle
    for shape_corner in shape {
        if shape_corner.x > c1.x.min(c2.x)
            && shape_corner.x < c1.x.max(c2.x)
            && shape_corner.y > c1.y.min(c2.y)
            && shape_corner.y < c1.y.max(c2.y)
        {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let corners: Vec<Corner> = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(",")
                .map(|num| num.trim().parse().unwrap())
                .collect();
            Corner::new(coords[0], coords[1])
        })
        .collect::<Vec<Corner>>();

    // Find the biggest area formed between any two corners
    let mut biggest_area: u64 = 0;
    for i in 0..corners.len() {
        for j in i + 1..corners.len() {
            let area = corners[i].area_with_other(&corners[j]);
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }
    Some(biggest_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let corners: Vec<Corner> = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(",")
                .map(|num| num.trim().parse().unwrap())
                .collect();
            Corner::new(coords[0], coords[1])
        })
        .collect::<Vec<Corner>>();

    // Find the biggest area formed between any two corners
    let mut biggest_area: u64 = 0;
    for i in 0..corners.len() {
        for j in i + 1..corners.len() {
            if rectangle_goes_outside_bounds(&corners[i], &corners[j], &corners) {
                continue;
            }

            let area = corners[i].area_with_other(&corners[j]);
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }
    Some(biggest_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    #[ignore]
    fn test_point_inside_polygon() {
        let mut polygon = Vec::new();
        polygon.push(Corner::new(1, 1));
        polygon.push(Corner::new(5, 1));
        polygon.push(Corner::new(5, 5));
        polygon.push(Corner::new(1, 5));

        let inside_point = Corner::new(3, 3);
        let outside_point = Corner::new(6, 3);

        assert!(point_inside_polygon(&inside_point, &polygon));
        assert!(!point_inside_polygon(&outside_point, &polygon));
    }

    #[test]
    fn test_rectangle_goes_outside_bounds() {
        let mut shape = Vec::new();

        //   1234567
        //01 .......
        //02 .......
        //03 .......
        //04 .#.#...
        //05 ##.##..
        //06 .......
        //07 .......
        //08 .......
        //09 .......
        //10 #...#..
        //11 .......
        //
        shape.push(Corner::new(1, 5));
        shape.push(Corner::new(2, 5));
        shape.push(Corner::new(2, 4));
        shape.push(Corner::new(4, 4));
        shape.push(Corner::new(4, 5));
        shape.push(Corner::new(5, 5));
        shape.push(Corner::new(5, 10));
        shape.push(Corner::new(1, 10));

        let c1 = Corner::new(2, 5);
        let c2 = Corner::new(5, 10);
        assert!(!rectangle_goes_outside_bounds(&c1, &c2, &shape));

        let c1 = Corner::new(1, 5);
        let c2 = Corner::new(4, 4);
        assert!(rectangle_goes_outside_bounds(&c1, &c2, &shape));
    }

    #[test]
    fn test_rectangle_goes_outside_concave() {
        let mut shape = Vec::new();

        //   1234567
        //01 .......
        //02 .......
        //03 .......
        //04 .......
        //05 ##.##..
        //06 .......
        //07 .#.#...
        //08 .......
        //09 .......
        //10 #...#..
        //11 .......
        //
        shape.push(Corner::new(1, 5));
        shape.push(Corner::new(2, 5));
        shape.push(Corner::new(2, 7));
        shape.push(Corner::new(4, 7));
        shape.push(Corner::new(4, 5));
        shape.push(Corner::new(5, 5));
        shape.push(Corner::new(5, 10));
        shape.push(Corner::new(1, 10));

        let c1 = Corner::new(2, 5);
        let c2 = Corner::new(5, 10);
        assert!(rectangle_goes_outside_bounds(&c1, &c2, &shape));

        let c1 = Corner::new(1, 5);
        let c2 = Corner::new(2, 7);
        assert!(!rectangle_goes_outside_bounds(&c1, &c2, &shape));
    }
}
