use std::collections::HashSet;

const INPUT: &str = include_str!("./input10.txt");
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn get_data() -> Vec<Vec<i32>> {
    INPUT
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid(x: i32, y: i32, lenx: usize, leny: usize) -> bool {
    x >= 0 && y >= 0 && x < lenx as i32 && y < leny as i32
}

fn score(coord: (usize, usize), grid: &Vec<Vec<i32>>, set: &mut HashSet<(usize, usize)>) {
    if grid[coord.0][coord.1] == 9 {
        set.insert(coord);
    } else {
        for d in DIRECTIONS {
            let (x, y) = (coord.0 as i32 + d.0, coord.1 as i32 + d.1);
            if is_valid(x, y, grid.len(), grid[0].len()) {
                if grid[x as usize][y as usize] == grid[coord.0][coord.1] + 1 {
                    score((x as usize, y as usize), grid, set);
                }
            }
        }
    }
}

fn score_part_two(coord: (usize, usize), grid: &Vec<Vec<i32>>) -> i32 {
    if grid[coord.0][coord.1] == 9 {
        1
    } else {
        let mut total = 0;
        for d in DIRECTIONS {
            let (x, y) = (coord.0 as i32 + d.0, coord.1 as i32 + d.1);
            if is_valid(x, y, grid.len(), grid[0].len()) {
                if grid[x as usize][y as usize] == grid[coord.0][coord.1] + 1 {
                    total += score_part_two((x as usize, y as usize), grid);
                }
            }
        }
        total
    }
}
pub fn part_one() -> i32 {
    let mut total = 0;
    let data = get_data();
    for row in 0..data.len() {
        for column in 0..data[0].len() {
            if data[row][column] == 0 {
                let mut h = HashSet::new();
                score((row, column), &data, &mut h);
                total += h.len() as i32;
            }
        }
    }
    total
}

pub fn part_two() -> i32 {
    let mut total = 0;
    let data = get_data();
    for row in 0..data.len() {
        for column in 0..data[0].len() {
            if data[row][column] == 0 {
                total += score_part_two((row, column), &data);
            }
        }
    }
    total
}
