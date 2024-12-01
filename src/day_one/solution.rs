use std::fs::File;
use std::io::{Error, Read};

pub fn solution() {
    let (list_a, list_b) = parse_input("input.txt").unwrap();
    let answer = sort_zip_abs_sum(list_a, list_b).to_string();
    println!("{}", answer);
}

fn parse_input(file_path: &str) -> Result<(Vec<i64>, Vec<i64>), Error> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let numbers: Vec<i64> = contents
        .split_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();

    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();

    for (i, &value) in numbers.iter().enumerate() {
        if i % 2 == 0 {
            a.push(value);
        } else {
            b.push(value);
        }
    }
    Ok((a, b))
}

pub fn sort_zip_abs_sum(mut a: Vec<i64>, mut b: Vec<i64>) -> u64 {
    a.sort();
    b.sort();

    std::iter::zip(a, b).map(|e| e.0.abs_diff(e.1)).sum()
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use std::io::Write;

    use super::*;

    #[test]
    fn day_one_example_should_be_11() {
        let first = vec![3, 4, 2, 1, 3, 3];
        let second = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(sort_zip_abs_sum(first.to_vec(), second.to_vec()), 11);
    }
    #[test]
    fn day_one_abs() {
        let a = vec![10];
        let b = vec![1];

        assert_eq!(sort_zip_abs_sum(a.to_vec(), b.to_vec()), 9);
        assert_eq!(sort_zip_abs_sum(b.to_vec(), a.to_vec()), 9);
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
