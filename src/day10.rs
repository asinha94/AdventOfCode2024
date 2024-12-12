use std::{collections::HashSet, fs};

struct Map {
    grid: Vec<Vec<u32>>
}

impl Map {
    fn get_score(&self) -> usize {
        let zeroes: Vec<_> = self.grid.iter()
            .enumerate()
            .map(|(y, row)| row.iter()
                .enumerate()
                .filter_map(|(x, n)| (*n == 0).then_some((x as i32,y as i32)))
                .collect::<Vec<_>>()
            ).flatten()
            .collect();

        zeroes.iter()
            .map(|(x,y)| self.recurse_helper(*x, *y, 0))
            .sum()
    }

    fn get_rating(&self) -> usize {
        let zeroes: Vec<_> = self.grid.iter()
            .enumerate()
            .map(|(y, row)| row.iter()
                .enumerate()
                .filter_map(|(x, n)| (*n == 0).then_some((x as i32,y as i32)))
                .collect::<Vec<_>>()
            ).flatten()
            .collect();

        zeroes.iter()
            .map(|(x,y)| self.recurse_down_path_ratings(*x, *y, 0))
            .sum()
    }

    fn inside_grid(&self, x: i32, y: i32) -> bool {
        let y_max = self.grid.len() as i32;
        let x_max = self.grid[0].len() as i32;
        x >= 0 && y >= 0 && y < y_max && x < x_max 
    }

    fn recurse_helper(&self, x: i32, y: i32, num: u32) -> usize {
        let mut seen = HashSet::new();
        self.recurse_down_path(x, y, num, &mut seen);
        seen.len()
    }

    fn recurse_down_path(&self, x: i32, y: i32, num: u32, seen: &mut HashSet<(i32, i32)>) {
        if !self.inside_grid(x, y) {
            return;
        }

        if self.grid[y as usize][x as usize] != num {
            return;
        }

        if num == 9 {
            seen.insert((x,y));
            return;
        }

        let next = num + 1;
        self.recurse_down_path(x-1, y, next, seen);
        self.recurse_down_path(x+1, y, next, seen);
        self.recurse_down_path(x, y-1, next, seen);
        self.recurse_down_path(x, y+1, next, seen);
    }

    fn recurse_down_path_ratings(&self, x: i32, y: i32, num: u32) -> usize {
        if !self.inside_grid(x, y) {
            return 0;
        }

        if self.grid[y as usize][x as usize] != num {
            return 0;
        }

        if num == 9 {
            return 1;
        }

        let next = num + 1;
        self.recurse_down_path_ratings(x-1, y, next) +
        self.recurse_down_path_ratings(x+1, y, next) +
        self.recurse_down_path_ratings(x, y-1, next) +
        self.recurse_down_path_ratings(x, y+1, next)
    }
}




pub fn part1() {
    let grid: Vec<Vec<_>> = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day10.txt").unwrap()
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        }).collect();

    let map = Map{grid};
    let score = map.get_score();
    let rating = map.get_rating();
    println!("Score: {score}");
    println!("Score: {rating}");
    
}