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

// Return true if the point is on the boundary of a polygon
fn point_on_boundary(point: &Corner, polygon: &Vec<Corner>) -> bool {
    let n = polygon.len();
    for i in 0..n {
        let a = &polygon[i];
        let b = &polygon[(i + 1) % n];

        // Check if point is on the line segment ab
        let cross_product = (point.y - a.y) * (b.x - a.x) - (point.x - a.x) * (b.y - a.y);
        if cross_product != 0 {
            continue; // Not collinear
        }

        let dot_product = (point.x - a.x) * (b.x - a.x) + (point.y - a.y) * (b.y - a.y);
        if dot_product < 0 {
            continue; // Point is before a
        }

        let squared_length_ab = (b.x - a.x).pow(2) + (b.y - a.y).pow(2);
        if dot_product > squared_length_ab {
            continue; // Point is after b
        }

        return true; // Point is on the segment
    }
    false
}

// Retrun true if the point is strictly inside the polygon
fn point_inside_polygon(point: &Corner, polygon: &Vec<Corner>) -> bool {
    // Use ray-casting algorithm
    let n = polygon.len();
    let mut inside = false;
    for i in 0..n {
        let a = &polygon[i];
        let b = &polygon[(i + 1) % n];

        // Ignore horizontal edges
        if a.y == b.y {
            continue;
        }

        // Does our ray intersect the edge?
        // Since we ignore horizontal edges, we can assume a.x == b.x
        // And we only need to check if point.y is between a.y and b.y (inclusive)
        if (point.y >= a.y.min(b.y)) && (point.y < a.y.max(b.y) && (point.x < a.x)) {
            inside = !inside;
        }
    }
    inside
}

fn point_inside_rectangle(point: &Corner, c1: &Corner, c2: &Corner) -> bool {
    let min_x = c1.x.min(c2.x);
    let max_x = c1.x.max(c2.x);
    let min_y = c1.y.min(c2.y);
    let max_y = c1.y.max(c2.y);

    point.x > min_x && point.x < max_x && point.y > min_y && point.y < max_y
}

fn rectangle_goes_outside_bounds(c1: &Corner, c2: &Corner, shape: &Vec<Corner>) -> bool {
    // Return true if any corner of the rectangle is outside the shape or any edge of the
    // shape is strictly inside the rectangle
    //
    // Make the other corners of the rectangle
    let c3 = Corner::new(c1.x, c2.y);
    let c4 = Corner::new(c2.x, c1.y);

    // The original points are on the shape by definition, so only need to check the other two
    // For each of the two corners, first check if they are exactly on an edge of the polygon
    if (!point_on_boundary(&c3, shape) && !point_inside_polygon(&c3, shape))
        || !point_on_boundary(&c4, shape) && !point_inside_polygon(&c4, shape)
    {
        return true;
    }

    for shape_corner in shape {
        if point_inside_rectangle(shape_corner, c1, c2) {
            return true;
        }
    }

    // Finally, if any vertical edge of the rectangle intersects any horizontal edge of the
    // polygon, (exclusing corners), then the rectangle goes outside the polygon
    for i in 0..shape.len() {
        let a = &shape[i];
        let b = &shape[(i + 1) % shape.len()];

        // Check if edge ab is horizontal
        if a.y == b.y {
            // Check vertical edges of rectangle
            let rect_left_x = c1.x.min(c2.x);
            let rect_right_x = c1.x.max(c2.x);
            let rect_bottom_y = c1.y.min(c2.y);
            let rect_top_y = c1.y.max(c2.y);

            // Left edge
            if (rect_left_x > a.x.min(b.x))
                && (rect_left_x < a.x.max(b.x))
                && (a.y > rect_bottom_y)
                && (a.y < rect_top_y)
            {
                return true;
            }

            // Right edge
            if (rect_right_x > a.x.min(b.x))
                && (rect_right_x < a.x.max(b.x))
                && (a.y > rect_bottom_y)
                && (a.y < rect_top_y)
            {
                return true;
            }
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_wrong_shape_with_real_input() {
        let input = advent_of_code::template::read_file("inputs", DAY);
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

        // From debug output, we know a rectangle that should not be valid
        let c1 = Corner::new(17454, 85504);
        let c2 = Corner::new(82409, 14643);

        assert!(rectangle_goes_outside_bounds(&c1, &c2, &corners));
    }

    #[test]
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
