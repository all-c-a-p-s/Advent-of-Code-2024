const INPUT: &str = include_str!("./input03.txt");

enum StringState {
    Invalid,
    Incomplete,
    Complete,
}

fn check(s: &String) -> StringState {
    match s.len() {
        0 => StringState::Incomplete,
        1 => {
            if *s == *"m" {
                StringState::Incomplete
            } else {
                StringState::Invalid
            }
        }
        2 => {
            if *s == *"mu" {
                StringState::Incomplete
            } else {
                StringState::Invalid
            }
        }
        3 => {
            if *s == *"mul" {
                StringState::Incomplete
            } else {
                StringState::Invalid
            }
        }
        4 => {
            if *s == *"mul(" {
                StringState::Incomplete
            } else {
                StringState::Invalid
            }
        }
        _ => {
            if s[0..4] != *"mul(" {
                return StringState::Invalid;
            }

            let (mut num1, mut num2) = (String::new(), String::new());
            let mut reached_comma = false;
            for i in 4..s.len() {
                let c = s.chars().collect::<Vec<char>>()[i];
                match c {
                    ',' => {
                        if num1.is_empty() {
                            return StringState::Invalid;
                        }
                        reached_comma = true;
                    }
                    x if x.is_ascii_digit() => {
                        if reached_comma {
                            num2 += String::from(c).as_str();
                        } else {
                            num1 += String::from(c).as_str();
                        }
                    }
                    ')' => {
                        if !num1.is_empty() && !num2.is_empty() && reached_comma {
                            return StringState::Complete;
                        } else {
                            return StringState::Invalid;
                        }
                    }
                    _ => return StringState::Invalid,
                }
            }
            StringState::Incomplete
            //the string won't necessarily be complete
        }
    }
}

fn parse_multiplication(mul: &str) -> i32 {
    let nums = mul
        .split(',')
        .map(|x| {
            x.chars()
                .filter(|y| y.is_ascii_digit())
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();
    nums[0] * nums[1]
}

fn get_lines() -> Vec<String> {
    String::from(INPUT)
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
}

pub fn part_one() -> i32 {
    let lines = get_lines();

    let mut valid_multiplications = Vec::new();
    for line in lines {
        let mut current_string = String::new();
        for c in line.chars() {
            current_string += String::from(c).as_str();
            match check(&current_string) {
                StringState::Invalid => current_string = String::new(),
                StringState::Incomplete => {}
                StringState::Complete => {
                    valid_multiplications.push(current_string);
                    current_string = String::new();
                }
            }
        }
    }

    valid_multiplications
        .iter()
        .fold(0, |acc, x| acc + parse_multiplication(x))
}

fn check_do_instruction(s: &str) -> StringState {
    match s {
        "" => StringState::Incomplete,
        "d" => StringState::Incomplete,
        "do" => StringState::Incomplete,
        "do(" => StringState::Incomplete,
        "don" => StringState::Incomplete,
        "don'" => StringState::Incomplete,
        "don't" => StringState::Incomplete,
        "don't(" => StringState::Incomplete,
        "don't()" => StringState::Complete,
        "do()" => StringState::Complete,
        _ => StringState::Invalid,
    }
}

pub fn part_two() -> i32 {
    let lines = get_lines();

    let mut ignore_current = false;
    let mut valid_multiplications = Vec::new();
    for line in lines {
        let (mut current_string, mut do_string) = (String::new(), String::new());
        for c in line.chars() {
            current_string += String::from(c).as_str();
            match check(&current_string) {
                StringState::Invalid => current_string = String::new(),
                StringState::Incomplete => {}
                StringState::Complete => {
                    if !ignore_current {
                        valid_multiplications.push(current_string);
                    }
                    current_string = String::new();
                }
            }

            do_string += String::from(c).as_str();
            match check_do_instruction(&do_string) {
                StringState::Invalid => do_string = String::new(),
                StringState::Incomplete => {}
                StringState::Complete => {
                    ignore_current = do_string.chars().collect::<Vec<char>>()[2] == 'n'
                }
            }
        }
    }

    valid_multiplications
        .iter()
        .fold(0, |acc, x| acc + parse_multiplication(x))
}
