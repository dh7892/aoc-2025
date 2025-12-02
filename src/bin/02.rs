advent_of_code::solution!(2);

use rayon::prelude::*;

// Return true if the first half of the digits in the number are the same as the second half
fn has_repeat(num: usize) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    if &num_str[..half] == &num_str[half..] {
        return true;
    }
    false
}

// Return true if and substring of digits repeats for the whole number
// The following are examples:
// 1212 -> true
// 1111 -> true
// 121212 -> true (12 repeats 3 times)
// 121233 -> false (12 repeats only twice, 33 does not repeat)
fn has_repeats(num: usize) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();
    if len < 2 {
        return false;
    }
    // We build a list of options for the repeating substring
    // by taking longer slices of the number from the start up to half the length
    let options = (1..=len / 2)
        .map(|l| num_str[0..l].parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    for trial in options {
        let trial_str = trial.to_string();
        let trial_len = trial_str.len();
        if len % trial_len != 0 {
            continue;
        }
        let mut repeated = true;
        for i in (0..len).step_by(trial_len) {
            if &num_str[i..i + trial_len] != trial_str {
                repeated = false;
                break;
            }
        }
        if repeated {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split(',').collect::<Vec<&str>>();
    let result = ranges
        .par_iter()
        .map(|s| {
            let r = s.split('-').collect::<Vec<&str>>();
            let start: usize = r[0].parse().unwrap();
            let end: usize = r[1].parse().unwrap();
            (start..=end).filter(|&n| has_repeat(n)).sum::<usize>()
        })
        .sum::<usize>();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.split(',').collect::<Vec<&str>>();
    let result = ranges
        .par_iter()
        .map(|s| {
            let r = s.split('-').collect::<Vec<&str>>();
            let start: usize = r[0].parse().unwrap();
            let end: usize = r[1].parse().unwrap();
            (start..=end).filter(|&n| has_repeats(n)).sum::<usize>()
        })
        .sum::<usize>();
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_repeat() {
        assert_eq!(has_repeat(1212), true);
        assert_eq!(has_repeat(1234), false);
        assert_eq!(has_repeat(1122), false);
        assert_eq!(has_repeat(1111), true);
        assert_eq!(has_repeat(123321), false);
        assert_eq!(has_repeat(123456), false);
    }

    #[test]
    fn test_has_repeats() {
        assert_eq!(has_repeats(1212), true);
        assert_eq!(has_repeats(1234), false);
        assert_eq!(has_repeats(1122), false);
        assert_eq!(has_repeats(1111), true);
        assert_eq!(has_repeats(123123), true);
        assert_eq!(has_repeats(123456), false);
        assert_eq!(has_repeats(121212), true);
        assert_eq!(has_repeats(121233), false);
        assert_eq!(has_repeats(123123123), true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
