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

fn part1_helper(grid: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    if grid[y as usize][x as usize] != 'X' {
        return 0;
    }

    let lower_y_bound = 0;
    let lower_x_bound = 0;
    let upper_y_bound = (grid.len() - 1) as i32;
    let upper_x_bound = (grid[0].len() - 1) as i32;

    let mut count = 0;
    for (yi, xi) in DIRECTIONS {
        for (i, c) in PATTERN.char_indices() {
            /* Create our vectors */
            let yy = y + (yi * i as i32);
            let xx = x + (xi * i as i32);

            if yy < lower_y_bound || yy > upper_y_bound {
                break;
            }

            if xx < lower_x_bound || xx > upper_x_bound {
                break;
            }

            let yyy = yy as usize;
            let xxx = xx as usize;
            if grid[yyy][xxx] != c {
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
                .map(|(x, _)| part1_helper(&grid, x as i32, y as i32))
                .sum::<i32>())
        .sum();

    println!("Total XMASes is {xmases}")
}


fn is_xmas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {

    if grid[y as usize][x as usize] != 'A' {
        return false;
    }

    let upper_y_bound = (grid.len() - 1) as i32;
    let upper_x_bound = (grid[0].len() - 1) as i32;

    const DIRECTIONS: [((i32, i32), (i32, i32)); 2] = [
        ((-1,-1),(1, 1)), // top-left, bottom-right
        ((-1, 1),(1,-1))  // top-right, bottom-left
    ];

    for ((yt,xt),(yb, xb)) in DIRECTIONS.iter() {
        let xxt = x + xt;
        let yyt = y + yt;

        let xxb = x + xb;
        let yyb = y + yb;

        if xxt < 0 || yyt < 0 || xxb > upper_x_bound || yyb > upper_y_bound {
            return false;
        }

        let m_or_s = grid[yyt as usize][xxt as usize];
        let s_or_m = match m_or_s {
            'M' => 'S',
            'S' => 'M',
            _ => return false
        };

        if grid[yyb as usize][xxb as usize] != s_or_m {
            return false;
        }
    }

    true
}

pub fn part2() {
    let grid: Vec<_> = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day04.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();


        let xmases: usize = grid
        .iter()
        .enumerate()
        .map(|(y, row)|
            row.iter()
                .enumerate()
                .filter(|(x, _)| {is_xmas(&grid, *x as i32, y as i32)})
                .count())
        .sum();

        println!("Xmas count: {xmases}");
}