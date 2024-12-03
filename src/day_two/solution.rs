pub fn solution_day_two() {
    let input = include_str!("input.txt");
    let count = part_two(input);
    println!("{}", count);
}

fn prepare(file: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in file.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap_or_else(|_| panic!("{}", x)))
            .collect();

        reports.push(numbers);
    }

    reports
}

fn is_increasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[1] > w[0])
}

fn is_decreasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[1] < w[0])
}

fn is_safe(arr: &[i32]) -> bool {
    arr.windows(2)
        .all(|w| (w[0] - w[1]).abs() >= 1 && (w[0] - w[1]).abs() <= 3)
}

fn is_monotonic(arr: &[i32]) -> bool {
    is_increasing(arr) || is_decreasing(arr)
}

pub fn part_one(file: &str) -> i32 {
    let reports = prepare(file);

    let mut count = 0;

    for report in reports {
        if is_monotonic(&report) && is_safe(&report) {
            count += 1;
        }
    }
    count
}

fn check_report_damp(rep: &mut Vec<i32>) -> bool {
    for i in 0..rep.len() {
        let removed = rep.remove(i);

        if is_monotonic(&rep) && is_safe(&rep) {
            return true;
        }
        rep.insert(i, removed);
    }

    false
}

pub fn part_two(file: &str) -> i32 {
    let reports = prepare(file);

    let mut count = 0;

    for mut report in reports {
        if check_report_damp(&mut report) {
            count += 1
        }
    }

    count
}
