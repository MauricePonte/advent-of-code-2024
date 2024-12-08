use std::collections::HashMap;

pub fn solution_day_seven() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let input = parse_input(input);
    let answer: Vec<i64> = input
        .iter()
        .map(|(&key, numbers)| {
            if can_calculate_part_one(key, numbers, 0, 0) {
                return key;
            } else {
                return 0;
            }
        })
        .collect();

    println!("Part one: {:?}", answer.into_iter().sum::<i64>());
}

fn part_two(input: &str) {
    let input = parse_input(input);
    let answer: Vec<i64> = input
        .iter()
        .map(|(&key, numbers)| {
            if can_calculate_part_two(key, numbers, 0, 0) {
                return key;
            } else {
                return 0;
            }
        })
        .collect();

    println!("Part two: {:?}", answer.into_iter().sum::<i64>());
}

fn parse_input(input: &str) -> HashMap<i64, Vec<i64>> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let split = line.split_once(':').unwrap();
        let key = split.0.parse::<i64>().unwrap();
        let value = split
            .1
            .split_whitespace()
            .filter_map(|num| num.parse::<i64>().ok())
            .collect();
        acc.insert(key, value);
        acc
    })
}

fn can_calculate_part_one(
    target: i64,
    numbers: &Vec<i64>,
    index: usize,
    current_result: i64,
) -> bool {
    if index == numbers.iter().len() {
        return current_result == target;
    }

    if can_calculate_part_one(target, numbers, index + 1, current_result + numbers[index]) {
        return true;
    }

    if can_calculate_part_one(target, numbers, index + 1, current_result * numbers[index]) {
        return true;
    }

    return false;
}

fn can_calculate_part_two(
    target: i64,
    numbers: &Vec<i64>,
    index: usize,
    current_result: i64,
) -> bool {
    if index == numbers.iter().len() {
        return current_result == target;
    }

    if can_calculate_part_two(target, numbers, index + 1, current_result + numbers[index]) {
        return true;
    }

    if can_calculate_part_two(target, numbers, index + 1, current_result * numbers[index]) {
        return true;
    }

    if can_calculate_part_two(
        target,
        numbers,
        index + 1,
        concat(current_result, numbers[index]),
    ) {
        return true;
    }

    return false;
}

fn concat(x: i64, y: i64) -> i64 {
    let concatenated = format!("{}{}", x, y);
    concatenated.parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst() {
        let target = 292;
        let numbers = vec![11, 6, 16, 20];
        let answer = can_calculate_part_one(target, &numbers, 0, 0);
        assert!(answer);
    }

    #[test]
    fn tst_neg() {
        let target = 83;
        let numbers = vec![17, 5];
        let answer = can_calculate_part_one(target, &numbers, 0, 0);
        assert!(!answer);
    }

    #[test]
    fn tst_part_two() {
        let target = 156;
        let numbers = vec![15, 6];
        assert_eq!(target, concat(numbers[0], numbers[1]));
        let answer = can_calculate_part_two(target, &numbers, 0, 0);
        assert!(answer);
    }
}
