advent_of_code::solution!(5);

fn item_in_ranges(item: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for (start, end) in ranges {
        if item >= *start && item <= *end {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    // Build a vector of inclusive ranges (each line is "a-b"
    let ranges = sections[0]
        .lines()
        .map(|line| {
            let parts = line.split('-').collect::<Vec<&str>>();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();
    // Now get our items to check
    let items_in_ranges = sections[1]
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|item| item_in_ranges(*item, &ranges))
        .count();

    Some(items_in_ranges as u64)
}

enum RangeOverlap {
    NoOverlap,
    BEndsInA,
    BBeginsInA,
    BInsideA,
    AInsideB,
}

trait RangeOverlapExt {
    fn overlap_with(&self, other: &Self) -> RangeOverlap;
}

impl RangeOverlapExt for (u64, u64) {
    fn overlap_with(&self, other: &(u64, u64)) -> RangeOverlap {
        let (a_start, a_end) = self;
        let (b_start, b_end) = other;

        if *b_end < *a_start || *b_start > *a_end {
            RangeOverlap::NoOverlap
        } else if *b_start <= *a_start && *b_end >= *a_start && *b_end <= *a_end {
            RangeOverlap::BEndsInA
        } else if *b_start >= *a_start && *b_start <= *a_end && *b_end >= *a_end {
            RangeOverlap::BBeginsInA
        } else if *b_start >= *a_start && *b_end <= *a_end {
            RangeOverlap::BInsideA
        } else if *b_start <= *a_start && *b_end >= *a_end {
            RangeOverlap::AInsideB
        } else {
            RangeOverlap::NoOverlap // Should not reach here
        }
    }
}

// Given a vec of inclusive ranges, and a new inclusive range,
// return a vec of inclusive ranges with the new range merged in
// so that the result has no overlapping ranges
fn merge_range_into_ranges(ranges: &Vec<(u64, u64)>, new_range: (u64, u64)) -> Vec<(u64, u64)> {
    let mut result = Vec::new();
    let mut merged_range = new_range;

    for &(start, end) in ranges {
        let overlap = merged_range.overlap_with(&(start, end));

        match overlap {
            RangeOverlap::NoOverlap => result.push((start, end)),
            RangeOverlap::BEndsInA => {
                merged_range.0 = start;
            }
            RangeOverlap::BBeginsInA => {
                merged_range.1 = end;
            }
            RangeOverlap::BInsideA => {
                // Do nothing, B is inside A
            }
            RangeOverlap::AInsideB => {
                merged_range = (start, end);
            }
        }
    }

    result.push(merged_range);
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    // Build a vector of inclusive ranges (each line is "a-b"
    let ranges = sections[0]
        .lines()
        .map(|line| {
            let parts = line.split('-').collect::<Vec<&str>>();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for range in ranges {
        merged_ranges = merge_range_into_ranges(&merged_ranges, range);
    }

    // Now just add up the total span of all ranges
    let items_in_ranges: u64 = merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    Some(items_in_ranges as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_range_into_ranges() {
        let ranges = vec![(1, 5), (10, 15), (20, 25)];
        let new_range = (12, 22);
        let merged = merge_range_into_ranges(&ranges, new_range);
        assert_eq!(merged.len(), 2);
        assert!(merged.contains(&(1, 5)));
        assert!(merged.contains(&(10, 25)));

        let ranges = vec![(1, 3), (5, 7)];
        let new_range = (3, 5);
        let merged = merge_range_into_ranges(&ranges, new_range);
        assert_eq!(merged.len(), 1);
        assert!(merged.contains(&(1, 7)));

        let ranges = vec![(1, 3), (7, 10)];
        let new_range = (5, 6);
        let merged = merge_range_into_ranges(&ranges, new_range);
        assert_eq!(merged.len(), 3);
        assert!(merged.contains(&(1, 3)));
        assert!(merged.contains(&(5, 6)));
        assert!(merged.contains(&(7, 10)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
