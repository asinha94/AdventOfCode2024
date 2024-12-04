use std::fs;

const PATTERN: &str = "XMAS";

const DIRECTIONS: [(i32, i32); 8] = [
        ( 0, 1), // E
        ( 0,-1), // W
        ( 1, 0), // S
        (-1, 0), // N
        ( 1, 1), // SE
        (-1,-1), // NW
        ( 1,-1), // SE
        (-1, 1)  // NE
    ];

fn part1_helper(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if grid[y][x] != 'X' {
        return 0;
    }

    let lower_y_bound = 0;
    let lower_x_bound = 0;
    let upper_y_bound = (grid.len() - 1) as i32;
    let upper_x_bound = (grid[0].len() - 1) as i32;

    let mut count = 0;
    for (yi, xi) in DIRECTIONS {
        for (i, c) in PATTERN.char_indices() {
            // cast
            let ii = i as i32;
            /* Create our vectors */
            let yy = (y as i32) + (yi * ii);
            let xx = (x as i32) + (xi * ii);

            if yy < lower_y_bound || yy > upper_y_bound {
                break;
            }

            if xx < lower_x_bound || xx > upper_x_bound {
                break;
            }

            let y_pos = yy as usize;
            let x_pos = xx as usize;

            if grid[y_pos][x_pos] != c {
                break;
            }

            if i == PATTERN.len() - 1 {
                count += 1;
            }
        }
    }

    count
}

pub fn part1() {
    let grid: Vec<_> = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day04.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let xmases: i32 = grid
        .iter()
        .enumerate()
        .map(|(y, row)| 
            row.iter()
                .enumerate()
                /* Only start at the Xs */
                .map(|(x, _)| part1_helper(&grid, x, y))
                .sum::<i32>())
        .sum();

    println!("Total XMASes is {xmases}")
}