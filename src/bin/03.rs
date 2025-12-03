advent_of_code::solution!(3);

// Return the largest digit and its index in [0..numbers.len()-n]
fn largest_digit_n_from_end(numbers: &[u64], n: usize) -> (u64, u64) {
    if numbers.len() <= n {
        return (0, 0);
    }
    let mut largest = 0;
    let mut largest_index = 0;
    for i in 0..numbers.len() - n {
        if numbers[i] > largest {
            largest = numbers[i];
            largest_index = i;
        }
    }
    (largest, largest_index as u64)
}

fn largest_n_digit_number_from_list(numbers: &[u64], n: usize) -> u64 {
    if numbers.len() < n {
        return 0;
    }
    let mut result = 0;
    let mut start_index = 0;
    for i in 0..n {
        let (largest, largest_index) = largest_digit_n_from_end(&numbers[start_index..], n - i - 1);
        result = result * 10 + largest;
        start_index += largest_index as usize + 1;
    }
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| {
            let numbers: Vec<u64> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|n| n as u64)
                .collect();
            largest_n_digit_number_from_list(&numbers, 2)
        })
        .sum::<u64>()
        .into();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| {
            let numbers: Vec<u64> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|n| n as u64)
                .collect();
            largest_n_digit_number_from_list(&numbers, 12)
        })
        .sum::<u64>()
        .into();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_number_from_list() {
        let numbers = vec![3, 5, 2, 7, 4];
        let result = largest_n_digit_number_from_list(&numbers, 2);
        assert_eq!(result, 74);

        let numbers = vec![9, 1, 8, 3];
        let result = largest_n_digit_number_from_list(&numbers, 2);
        assert_eq!(result, 98);

        let numbers = vec![1, 2, 3];
        let result = largest_n_digit_number_from_list(&numbers, 2);
        assert_eq!(result, 23);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
