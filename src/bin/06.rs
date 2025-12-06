advent_of_code::solution!(6);

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut data: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        if words[0] == "+" || words[0] == "*" {
            operators = words
                .iter()
                .map(|&w| {
                    if w == "+" {
                        Operator::Add
                    } else if w == "*" {
                        Operator::Multiply
                    } else {
                        panic!("Unexpected operator: {}", w);
                    }
                })
                .collect::<Vec<Operator>>();
        } else {
            // We have a row of numbers, so we can convert to a vec of ints
            let numbers = words
                .iter()
                .map(|&w| w.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            data.push(numbers);
        }
    }
    // So we now have a grid of data and a list of operators
    // We need to loop over columns and apply the operators to each column
    let mut col_results: Vec<u64> = data[0].clone();
    for row in 1..data.len() {
        for (col, value) in data[row].iter().enumerate() {
            match &operators[col] {
                Operator::Add => {
                    col_results[col] += value;
                }
                Operator::Multiply => {
                    col_results[col] *= value;
                }
            }
        }
    }

    // Now our final result is the sum of all column results
    Some(col_results.iter().sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    // Split the input into the last line and the rest
    let mut operators: Vec<Operator> = Vec::new();
    let mut char_array: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        if chars[0] == '+' || chars[0] == '*' {
            operators = chars
                .iter()
                // filter map to remove spaces and convet to operators
                .filter_map(|&c| {
                    if c == '+' {
                        Some(Operator::Add)
                    } else if c == '*' {
                        Some(Operator::Multiply)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Operator>>();
        } else {
            char_array.push(chars);
        }
    }

    let num_rows = char_array.len();
    let num_cols = char_array[0].len();

    // "rotate" our chars to we have a vec of columns
    let mut col_array: Vec<Vec<char>> = vec![Vec::new(); num_cols];
    for row in 0..num_rows {
        for col in 0..num_cols {
            col_array[col].push(char_array[row][col]);
        }
    }

    let mut operands: Vec<Vec<u64>> = Vec::new();
    let mut current_operands: Vec<u64> = Vec::new();
    for col in col_array {
        // join the chars into a string
        let s = col.iter().collect::<String>();
        if let Ok(num) = s.trim().parse::<u64>() {
            current_operands.push(num);
        } else {
            // We reached the end of the operands for this column
            operands.push(current_operands);
            current_operands = Vec::new();
        }
    }
    // push the last operands
    if !current_operands.is_empty() {
        operands.push(current_operands);
    }
    assert!(operands.len() == operators.len());
    let result = operands
        .iter()
        .enumerate()
        .map(|(i, ops)| {
            let op = &operators[i];
            match op {
                Operator::Add => ops.iter().sum::<u64>(),
                Operator::Multiply => ops.iter().product::<u64>(),
            }
        })
        .sum::<u64>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
