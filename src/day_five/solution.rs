use itertools::Itertools;

pub fn solution_day_five() {
    let rules_input = include_str!("input_1.txt");
    let update_input = include_str!("input_2.txt");
    //part_one(rules_input, update_input);
    part_two(rules_input, update_input);
}

fn part_one(rules_input: &str, update_input: &str) {
    let rules = parse_rules(rules_input);
    let updates = parse_updates(update_input);
    let result: i32 = updates
        .iter()
        .map(|update| {
            if check_rules(&update, &rules) {
                return get_middle(&update);
            } else {
                return 0;
            }
        })
        .sum();
    println!("The answer to part one is: {}", result)
}

fn part_two(rules_input: &str, update_input: &str) {
    let rules = parse_rules(rules_input);
    let updates = parse_updates(update_input);
    let incorrect_updates: Vec<Vec<i8>> = updates
        .into_iter()
        .filter(|update| !check_rules(&update, &rules))
        .collect();
    let result: i32 = incorrect_updates
        .into_iter()
        .map(|incorrect| {
            let correct = solve_incorrect(&incorrect, &rules);
            get_middle(&correct)
        })
        .sum();

    println!("The answer to part two is: {}", result)
}

fn parse_rules(input: &str) -> Vec<Rule> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let x: Vec<&str> = line.split('|').collect();
            Rule {
                x: x[0].parse::<i8>().unwrap(),
                y: x[1].parse::<i8>().unwrap(),
            }
        })
        .collect()
}

fn parse_updates(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i8>().unwrap())
                .collect_vec()
        })
        .collect()
}

fn check_rules(update: &Vec<i8>, rules: &Vec<Rule>) -> bool {
    rules.iter().all(|rule| {
        if !rule.applies(update) {
            return true;
        }
        rule.check_rule(&update)
    })
}

fn get_middle(update: &Vec<i8>) -> i32 {
    let mid = update.iter().count() / 2;
    println!("{}", mid);
    update[mid] as i32
}

fn solve_incorrect(update: &Vec<i8>, rules: &Vec<Rule>) -> Vec<i8> {
    if check_rules(update, &rules) {
        return update.to_owned();
    }

    for rule in rules
        .iter()
        .filter(|rule| rule.applies(update) && rule.check_rule(update))
    {
        let mut mutable_update = update.to_owned();
        return solve_incorrect(&rule.apply_rule(&mut mutable_update), &rules);
    }

    return update.to_owned();
}

#[derive(Debug)]
struct Rule {
    x: i8,
    y: i8,
}

impl Rule {
    pub fn check_rule(&self, update: &Vec<i8>) -> bool {
        let index_x = update.into_iter().position(|&n| n == self.x).unwrap();
        let index_y = update.into_iter().position(|&n| n == self.y).unwrap();
        index_x < index_y
    }

    pub fn apply_rule(&self, update: &mut Vec<i8>) -> Vec<i8> {
        println!("{:?}", self);
        println!("{:?}", update);
        let index_x = update.into_iter().position(|n| *n == self.x).unwrap();
        let index_y = update.into_iter().position(|n| *n == self.y).unwrap();
        update.swap(index_x, index_y);
        update.to_owned()
    }

    fn applies(&self, update: &Vec<i8>) -> bool {
        update.contains(&self.x) && update.contains(&self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_check_rule() {
        let rule = Rule { x: 2, y: 3 };
        let update: Vec<i8> = vec![1, 2, 3, 4];
        let result = rule.check_rule(&update);
        assert!(result);
    }

    #[test]
    fn rule_check_rule_ne() {
        let rule = Rule { x: 3, y: 2 };
        let update: Vec<i8> = vec![1, 2, 3, 4];
        let result = rule.check_rule(&update);
        assert!(!result);
    }

    #[test]
    fn rule_check_rule_ne() {
        let rule = Rule { x: 3, y: 2 };
        let update: Vec<i8> = vec![1, 2, 3, 4];
        let result = rule.check_rule(&update);
        assert!(!result);
    }
}
