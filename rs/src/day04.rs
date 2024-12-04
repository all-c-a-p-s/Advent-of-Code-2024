const INPUT: &str = include_str!("input04.txt");

fn get_grid() -> Vec<Vec<char>> {
    String::from(INPUT)
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}

fn check_in_grid(x: i32, y: i32, lenx: usize, leny: usize) -> bool {
    (x >= 0) && (x < lenx as i32) && (y >= 0) && (y < leny as i32)
}

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn cast_ray(d: (i32, i32), x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    let (mut next_x, mut next_y) = (x as i32 + d.1, y as i32 + d.0);
    if !check_in_grid(next_x, next_y, grid[0].len(), grid.len()) {
        return false;
    }

    if grid[next_y as usize][next_x as usize] == 'M' {
        (next_x, next_y) = (next_x as i32 + d.1, next_y as i32 + d.0);

        if !check_in_grid(next_x, next_y, grid[0].len(), grid.len()) {
            return false;
        }

        if grid[next_y as usize][next_x as usize] == 'A' {
            (next_x, next_y) = (next_x as i32 + d.1, next_y as i32 + d.0);

            if !check_in_grid(next_x, next_y, grid[0].len(), grid.len()) {
                return false;
            }
            return grid[next_y as usize][next_x as usize] == 'S';
        }
    }
    false
}

//cast a ray north, east, south, west, ne, etc. for all instances of X
//add 1 to count if you find a completed XMAS
pub fn part_one() -> i32 {
    let mut total = 0;
    let grid = get_grid();
    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            if grid[row][column] != 'X' {
                continue;
            }
            for d in DIRECTIONS {
                if cast_ray(d, column, row, &grid) {
                    total += 1;
                }
            }
        }
    }
    total
}

fn match_grid(grid: (char, char, char, char, char, char, char, char, char)) -> i32 {
    match grid {
        ('M', _, 'M', _, 'A', _, 'S', _, 'S') => 1,
        ('M', _, 'S', _, 'A', _, 'M', _, 'S') => 1,
        ('S', _, 'M', _, 'A', _, 'S', _, 'M') => 1,
        ('S', _, 'S', _, 'A', _, 'M', _, 'M') => 1,
        _ => 0,
    }
}

//pattern match on 3x3 grids
pub fn part_two() -> i32 {
    let mut total = 0;
    let grid = get_grid();
    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            //check opposite corner of rectangle
            if !check_in_grid(column as i32 + 2, row as i32 + 2, grid[0].len(), grid.len()) {
                continue;
            }
            let grid_3x3 = (
                grid[row][column],
                grid[row][column + 1],
                grid[row][column + 2],
                grid[row + 1][column],
                grid[row + 1][column + 1],
                grid[row + 1][column + 2],
                grid[row + 2][column],
                grid[row + 2][column + 1],
                grid[row + 2][column + 2],
            );
            total += match_grid(grid_3x3);
        }
    }
    total
}
