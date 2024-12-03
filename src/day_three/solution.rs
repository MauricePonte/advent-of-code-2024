pub fn solution_day_three() {
    let hay = include_str!("input.txt");
    println!("part one {}", part_one(&hay));
    println!("part two {}", part_two(&hay));
}

fn part_one(hay: &str) -> i32 {
    let re = regex::Regex::new(r"(mul\((\d{1,3})\,(\d{1,3})\))").unwrap();
    let mut result: Vec<i32> = vec![];
    for (_, [_, num_a, num_b]) in re.captures_iter(hay).map(|c| c.extract()) {
        let a = num_a.parse::<i32>().unwrap();
        let b = num_b.parse::<i32>().unwrap();
        result.push(a * b);
    }
    result.iter().sum()
}

fn part_two(hay: &str) -> i32 {
    let begin = "do()".to_string();
    let hayhay = format!("{begin}{hay}{begin}");

    let reg = fancy_regex::Regex::new(r"(don't\(\))([\s\S]*?)(do\(\))").unwrap();
    let alot_less_hay = reg.replace_all(&hayhay, "").to_string();

    part_one(&alot_less_hay)
}
