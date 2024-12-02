use std::{
    f32::consts::PI,
    fs::File,
    io::{Error, Read},
};

pub fn solution_day_two() {
    let data = parse_file("input.txt").expect("failed to read from file");
    let answer_part_one = amount_levels_are_safe(&data);
    println!("The answer to part one is: {}", answer_part_one);
}

fn parse_file(file_path: &str) -> Result<Vec<Vec<i64>>, Error> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut data: Vec<Vec<i64>> = vec![];
    contents.lines().for_each(|line| {
        data.push(
            line.split_whitespace()
                .filter_map(|split| split.parse::<i64>().ok())
                .collect(),
        )
    });

    Ok(data)
}

fn amount_levels_are_safe(data: &Vec<Vec<i64>>) -> usize {
    data.iter().filter(|v| is_safe(v)).count()
}

fn is_safe(v: &Vec<i64>) -> bool {
    let diff = vec_diff(&v);

    diff.iter().all(|&n| n > 0 && (1..=3).contains(&n))
        || diff.iter().all(|&n| n < 0 && (-3..=-1).contains(&n))
}

fn vec_diff(v: &Vec<i64>) -> Vec<i64> {
    let skip = v.iter().skip(1);
    v.iter().zip(skip).map(|(x, y)| x - y).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::io::Write;

    #[test]
    fn vec_diff_positive() {
        let a = vec![2, 1];
        assert_eq!(vec_diff(&a), vec!(1));
    }

    #[test]
    fn vec_diff_negative() {
        let a = vec![1, 2];
        assert_eq!(vec_diff(&a), vec!(-1));
    }

    #[test]
    fn is_safe_when_decreasing() {
        let v = vec![7, 6, 4, 2, 1];
        assert!(is_safe(&v));
    }

    #[test]
    fn is_safe_when_increasing() {
        let v = vec![1, 3, 6, 7, 9];
        assert!(is_safe(&v));
    }

    #[test]
    fn is_unsafe_when_increase() {
        let v = vec![1, 2, 7, 8, 9];
        assert!(!is_safe(&v));
    }

    #[test]
    fn is_unsafe_when_decrease() {
        let v = vec![9, 7, 6, 2, 1];
        assert!(!is_safe(&v));
    }

    #[test]
    fn is_unsafe_when_increase_nor_decrease() {
        let v = vec![1, 3, 2, 4, 5];
        assert!(!is_safe(&v));
    }

    #[test]
    fn is_unsafe_when_increase_and_decrease() {
        let v = vec![1, 3, 2, 4, 5];
        assert!(!is_safe(&v));
    }

    #[test]
    fn parse_file_parses_file() {
        let file_path = "test.txt";
        let content = "1 2 3
            1 2 3";
        let mut file = File::create(file_path).expect("failed to create test file");

        file.write_all(content.as_bytes())
            .expect("failed to write to file");

        let mut expected: Vec<Vec<i64>> = vec![];
        expected.push(vec![1, 2, 3]);
        expected.push(vec![1, 2, 3]);
        let actual = parse_file(file_path).unwrap();
        assert_eq!(actual, expected);

        remove_file(file_path).expect("failed to remove file");
    }
}