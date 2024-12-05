const INPUT: &str = include_str!("input05.txt");

fn get_data() -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut reached_split = false;
    let (mut rules, mut pages) = (Vec::new(), Vec::new());
    for line in INPUT.lines() {
        if line.is_empty() {
            reached_split = true;
            continue;
        }
        match reached_split {
            false => rules.push(line.split("|").map(|x| x.parse::<i32>().unwrap()).collect()),
            true => pages.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect()),
        }
    }
    (rules, pages)
}

fn get_relevant_rules(v: &Vec<i32>, rules: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    rules
        .iter()
        .filter(|x| v.contains(&x[0]) && v.contains(&x[1]))
        .map(|x| x.clone())
        .collect()
}

fn count_first(x: i32, rules: &Vec<Vec<i32>>) -> usize {
    rules
        .iter()
        .fold(0, |acc, y| acc + if y[0] == x { 1 } else { 0 })
}

fn is_valid(order: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    for rule in rules {
        if order.iter().position(|n| *n == rule[0]) > order.iter().position(|n| *n == rule[1]) {
            return false;
        }
    }
    true
}

pub fn part_one() -> i32 {
    let (rules, pages) = (get_data().0, get_data().1);
    let mut total = 0;
    for page in pages {
        let r = get_relevant_rules(&page, &rules);
        if is_valid(&page, &r) {
            let middle_index = (page.len() + 1) / 2 - 1;
            total += page[middle_index];
        }
    }
    total
}

pub fn part_two() -> i32 {
    let (rules, pages) = (get_data().0, get_data().1);
    let mut total = 0;
    for page in pages {
        let r = get_relevant_rules(&page, &rules);
        if is_valid(&page, &r) {
            continue;
        }
        let mut res = page.clone();
        res.sort_by(|a, b| count_first(*b, &r).cmp(&count_first(*a, &r)));
        let middle_index = (res.len() + 1) / 2 - 1;
        total += res[middle_index];
    }
    total
}
