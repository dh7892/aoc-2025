advent_of_code::solution!(1);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    steps: usize,
}

impl Move {
    fn from_str(s: &str) -> Option<Move> {
        let (dir_char, steps_str) = s.split_at(1);
        let direction = match dir_char {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return None,
        };
        let steps = steps_str.parse::<usize>().ok()?;
        Some(Move { direction, steps })
    }
    fn apply(&self, position: i32, num_positions: i32) -> i32 {
        // Apply the movement modulo the max position
        match self.direction {
            Direction::Left => (position - self.steps as i32).rem_euclid(num_positions),
            Direction::Right => (position + self.steps as i32).rem_euclid(num_positions),
        }
    }
    fn num_zero_landings(&self, position: i32, num_positions: i32) -> i32 {
        match self.direction {
            Direction::Left => {
                let times_round = self.steps as i32 / num_positions;
                let remainder = self.steps as i32 % num_positions;
                // Check if we cross zero in the remainder movement
                if remainder >= position && position != 0 {
                    times_round + 1
                } else {
                    times_round
                }
            }
            Direction::Right => (position + self.steps as i32).div_euclid(num_positions),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let answer = input
        .lines()
        .map(|line| Move::from_str(line).unwrap())
        .scan(50i32, |current, mv| {
            *current = mv.apply(*current, 100);
            Some(if *current == 0 { 1 } else { 0 })
        })
        .sum::<usize>();
    Some(answer as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let answer = input
        .lines()
        .map(|line| Move::from_str(line).unwrap())
        // Instead of reporting position, we now count 0 crossings
        .scan(50, |current, mv| {
            let num_crossings = mv.num_zero_landings(*current, 100);
            *current = mv.apply(*current, 100);
            Some(num_crossings)
        })
        .sum::<i32>();
    Some(answer as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_zero_crossings() {
        let mv_left = Move::from_str("L3").unwrap();
        let mv_right = Move::from_str("R4").unwrap();

        assert_eq!(mv_left.num_zero_landings(5, 10), 0);
        assert_eq!(mv_right.num_zero_landings(5, 10), 0);
        assert_eq!(mv_right.num_zero_landings(8, 10), 1);
        assert_eq!(mv_left.num_zero_landings(2, 10), 1);

        let big_mv_right = Move::from_str("R1000").unwrap();
        assert_eq!(big_mv_right.num_zero_landings(50, 100), 10);
    }

    #[test]
    fn test_move_from_zero() {
        let mv_left = Move::from_str("L3").unwrap();
        assert_eq!(mv_left.num_zero_landings(3, 10), 1);
        assert_eq!(mv_left.num_zero_landings(0, 10), 0);
    }

    #[test]
    fn test_apply_move() {
        let mv_left = Move::from_str("L3").unwrap();
        let mv_right = Move::from_str("R4").unwrap();

        assert_eq!(mv_left.apply(5, 10), 2);
        assert_eq!(mv_right.apply(5, 10), 9);
        assert_eq!(mv_right.apply(8, 10), 2);
        assert_eq!(mv_left.apply(2, 10), 9);
    }

    #[test]
    fn test_special_cases() {
        let mv = Move::from_str("L30").unwrap();
        assert_eq!(mv.apply(82, 100), 52);

        let mv = Move::from_str("L266").unwrap();
        assert_eq!(mv.apply(50, 100), 84);

        let mv = Move::from_str("L330").unwrap();
        assert_eq!(mv.apply(94, 100), 64);
    }

    #[test]
    fn test_bespoke_part_two() {
        let input = "R60\nL120\nR250\nL300\nR90";
        let result = part_two(input);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
