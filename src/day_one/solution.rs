use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use std::iter::zip;

pub fn solution() {
    let (list_a, list_b) = parse_input("input.txt").unwrap();
    let answer_part_one = distance_score(&list_a, &list_b).to_string();
    println!("The answer to part one is: {}", answer_part_one);
    let answer_part_two = similarity_score(&list_a, &list_b);
    println!("The answer to part two is: {}", answer_part_two);
}

fn parse_input(file_path: &str) -> Result<(Vec<i64>, Vec<i64>), Error> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let numbers: Vec<i64> = contents
        .split_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();

    let (a, b): (Vec<(usize, i64)>, Vec<(usize, i64)>) = numbers
        .into_iter()
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    Ok((
        a.into_iter().map(|n| n.1).collect(),
        b.into_iter().map(|n| n.1).collect(),
    ))
}

fn distance_score(a: &Vec<i64>, b: &Vec<i64>) -> u64 {
    let mut x = a.to_vec();
    x.sort();
    let mut y = b.to_vec();
    y.sort();

    zip(x, y).map(|e| e.0.abs_diff(e.1)).sum()
}

fn similarity_score(a: &Vec<i64>, b: &Vec<i64>) -> u64 {
    let mut occurances = HashMap::new();

    for &num in b {
        *occurances.entry(num).or_insert(0) += 1;
    }

    a.iter()
        .map(|n| (occurances.get(n).unwrap_or(&0) * n) as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use std::io::Write;

    use super::*;

    #[test]
    fn distance_score_example_returns_11() {
        let first = vec![3, 4, 2, 1, 3, 3];
        let second = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(distance_score(&first, &second), 11);
    }
    #[test]
    fn distance_score_abs() {
        let a = vec![10];
        let b = vec![1];

        assert_eq!(distance_score(&a, &b), 9);
        assert_eq!(distance_score(&b, &a), 9);
    }

    #[test]
    fn simularity_score_example_returns_31() {
        let first = vec![3, 4, 2, 1, 3, 3];
        let second = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(similarity_score(&first, &second), 31);
    }

    #[test]
    fn parse_file_parses_correctly() {
        let file_path = "test.txt";
        let content = "10   1";
        let mut file = File::create(file_path).expect("failed to create test file");

        file.write_all(content.as_bytes())
            .expect("failed to write to file");

        let expected: (Vec<i64>, Vec<i64>) = (vec![10], vec![1]);
        let actual = parse_input(file_path).unwrap();
        assert_eq!(actual, expected);

        remove_file(file_path).expect("failed to remove file");
    }
}
