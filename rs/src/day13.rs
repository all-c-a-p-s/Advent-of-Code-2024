const INPUT: &str = include_str!("./input13.txt");

struct Game {
    a: (i128, i128),
    b: (i128, i128),
    target: (i128, i128),
}

impl Game {
    fn from_lines(v: &Vec<&str>) -> Self {
        let l: Vec<Vec<i128>> = v
            .iter()
            .map(|x| {
                x.chars()
                    .filter(|y| y.is_ascii_digit() || *y == ' ')
                    .collect::<String>()
                    .trim()
                    .split_whitespace()
                    .map(|z| z.parse::<i128>().unwrap())
                    .collect()
            })
            .collect();
        Game {
            a: (l[0][0], l[0][1]),
            b: (l[1][0], l[1][1]),
            target: (l[2][0], l[2][1]),
        }
    }
}

fn get_data() -> Vec<Game> {
    let lines: Vec<&str> = INPUT.lines().collect();
    let (mut current_game, mut games) = (Vec::new(), Vec::new());

    for line in lines {
        if line.trim().is_empty() {
            games.push(Game::from_lines(&current_game));
            current_game = vec![]
        } else {
            current_game.push(line);
        }
    }
    games.push(Game::from_lines(&current_game));
    games
}

pub fn hcf(a: i128, b: i128) -> i128 {
    match b {
        0 => a,
        _ => hcf(b, a % b),
    }
}

pub fn lcm(a: i128, b: i128) -> i128 {
    (a * b) / hcf(a, b)
}

struct Equation {
    x_coef: i128,
    y_coef: i128,
    rhs: i128,
}

struct Solution {
    x: i128,
    y: i128,
}

impl Equation {
    fn multiply(&self, k: i128) -> Self {
        Equation {
            x_coef: self.x_coef * k,
            y_coef: self.y_coef * k,
            rhs: self.rhs * k,
        }
    }

    fn subtract(&self, other: &Equation) -> Self {
        Equation {
            x_coef: self.x_coef - other.x_coef,
            y_coef: self.y_coef - other.y_coef,
            rhs: self.rhs - other.rhs,
        }
    }

    //where self is the basic equation
    fn interpret(&self, other: &Equation) -> Option<Solution> {
        match (self.x_coef, self.y_coef) {
            (0, 0) => return None,
            (a, 0) => {
                if self.rhs % a != 0 {
                    return None;
                }
                let x = self.rhs / a;
                if (other.rhs - other.x_coef * x) % other.y_coef != 0 {
                    return None;
                }
                let y = (other.rhs - other.x_coef * x) / other.y_coef;
                return Some(Solution { x, y });
            }
            (0, b) => {
                if self.rhs % b != 0 {
                    return None;
                }
                let y = self.rhs / b;
                if (other.rhs - other.y_coef * y) % other.x_coef != 0 {
                    return None;
                }
                let x = (other.rhs - other.y_coef * y) / other.x_coef;
                return Some(Solution { x, y });
            }
            _ => panic!("interpret() called even though neither x nor y was basic"),
        }
    }

    fn from_game(g: &Game) -> (Self, Self) {
        let a = Equation {
            x_coef: g.a.0,
            y_coef: g.b.0,
            rhs: g.target.0,
        };
        let b = Equation {
            x_coef: g.a.1,
            y_coef: g.b.1,
            rhs: g.target.1,
        };
        (a, b)
    }
}

fn solve(e1: &Equation, e2: &Equation) -> Option<Solution> {
    let l = lcm(e1.x_coef, e2.x_coef);
    let (k1, k2) = (l / e1.x_coef, l / e2.x_coef);
    let (a, b) = (e1.multiply(k1), e2.multiply(k2));
    let c = a.subtract(&b);
    c.interpret(&a)
}

fn score(s: &Option<Solution>) -> i128 {
    match s {
        Some(k) => 3 * k.x + k.y,
        None => 0,
    }
}

fn minimum_tokens(g: &Game) -> i128 {
    let (e1, e2) = Equation::from_game(g);
    score(&solve(&e1, &e2))
}

pub fn part_one() -> i128 {
    let data = get_data();
    data.iter().fold(0, |acc, x| acc + minimum_tokens(x))
}
pub fn part_two() -> i128 {
    let mut data = get_data();
    for i in 0..data.len() {
        data[i].target.0 += 10_000_000_000_000;
        data[i].target.1 += 10_000_000_000_000;
    }

    data.iter().fold(0, |acc, x| acc + minimum_tokens(x))
}
