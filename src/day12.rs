use std::{collections::HashSet, fs, io::empty};

const DIRECTIONS: [(i32, i32); 4] = [
        (1 , 0),
        (-1, 0),
        (0 , 1),
        (0 ,-1)
];

fn out_of_bounds(x: i32, y: i32, height: i32, width: i32) -> bool {
    x < 0 || y < 0 || x >= width || y >= height
}


fn get_area_price(grid: &mut Vec<Vec<char>>, y: usize, x: usize, height: usize, width: usize) -> i32 {

    let c = grid[y][x];
    if c == ' ' {
        return 0;
    }

    let mut seen = HashSet::new();
    seen.insert((x,y));
    let mut surrounding = vec![(x, y)];
    let mut area = 0;
    let mut perimeter = 0;
    
    while surrounding.len() > 0 {

        let (xi, yi) = surrounding.pop().unwrap();
        area += 1;

        for (dx, dy) in DIRECTIONS {

            let x = xi as i32 + dx;
            let y = yi as i32 + dy;

            // Edge of the map
            if out_of_bounds(x, y, height as i32, width as i32) {
                perimeter += 1;
                continue;
            }

            // Not same type
            let coords: (usize, usize) = (x as usize, y as usize);
            if grid[coords.1][coords.0] != c {
                perimeter += 1;
                continue;
            }

            // Plant already counted
            if seen.contains(&coords) {
                continue;
            }

            surrounding.push(coords);
            seen.insert(coords);
            
        }
    }

    for (x, y) in seen {
        grid[y][x] = ' ';
    }
    
    //println!("{c}: {area} * {perimeter} = {}", area * perimeter);
    area * perimeter
}


pub fn part1() {
    let mut grid: Vec<Vec<_>> = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day12.txt").unwrap()
        .lines()
        .map(|row| row.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();
    let mut total_price = 0;
    for y in 0..height {
        for x in 0..width {
            total_price += get_area_price(&mut grid, y, x, height, width);
        }
    }

    println!("Total Price: {total_price}");
}

const UP: i32    = 1 << 0;
const DOWN: i32  = 1 << 1;
const LEFT: i32  = 1 << 2;
const RIGHT: i32 = 1 << 3;

const TOP_LEFT: i32 = UP | LEFT;
const TOP_RIGHT: i32 = UP | RIGHT;
const BOTTOM_LEFT: i32 = DOWN | LEFT;
const BOTTOM_RIGHT: i32 = DOWN | RIGHT;


const DIRECTIONS_ENUM: [(i32, i32, i32); 4] = [
        (1 , 0, RIGHT),
        (-1, 0, LEFT),
        (0 , 1, DOWN),
        (0 ,-1, UP)
];

fn inc_if_corner(empty_sides: i32, corner: i32) -> i32 {
    if empty_sides & corner == corner {
        return 1;
    }

    0
 }

 fn inc_if_interior(non_empty_sides: u32, corner: i32, c: char, x: i32, y: i32, grid: &Vec<Vec<char>>) -> i32 {
    
    let corner_u32 = corner as u32;
    if non_empty_sides & corner_u32 != corner_u32 {
        return 0;
    }

    // mathematically cannot be out of bounds if mask is applied
    // so don't check
    if grid[y as usize][x as usize] != c {1} else {0}
 }


fn get_area_price2(grid: &mut Vec<Vec<char>>, y: usize, x: usize, height: usize, width: usize) -> i32 {

    let c = grid[y][x];
    if c == ' ' {
        return 0;
    }

    let mut seen = HashSet::new();
    let mut surrounding = vec![(x, y)];
    let mut corners = 0;

    
    while surrounding.len() > 0 {

        let base_coords = surrounding.pop().unwrap();
        if seen.contains(&base_coords) {
            continue;
        }

        let (xi, yi) = base_coords;
        let mut empty_sides = 0;
        // Need this to find interior corners
        let mut same_type_sides: u32 = 0;

        for (dx, dy, dir) in DIRECTIONS_ENUM {

            let x = xi as i32 + dx;
            let y = yi as i32 + dy;

            // Edge of the map
            if out_of_bounds(x, y, height as i32, width as i32) {
                empty_sides |= dir;
                continue;
            }

            // Not same type
            let coords: (usize, usize) = (x as usize, y as usize);
            if grid[coords.1][coords.0] != c {
                empty_sides |= dir;
                continue;
            }

            // To handle case of interior sides, we see if on 2 adject sides have same type
            same_type_sides |= dir as u32;

            // Plant already counted
            if !seen.contains(&coords) {
                surrounding.push(coords);
            }
        }

        seen.insert((xi, yi));
        corners += inc_if_corner(empty_sides, TOP_LEFT);
        corners += inc_if_corner(empty_sides, TOP_RIGHT);
        corners += inc_if_corner(empty_sides, BOTTOM_LEFT);
        corners += inc_if_corner(empty_sides, BOTTOM_RIGHT);

        corners += inc_if_interior(same_type_sides, TOP_LEFT, c, xi as i32-1, yi as i32-1, &grid);
        corners += inc_if_interior(same_type_sides, BOTTOM_LEFT, c, xi as i32-1, yi as i32+1, &grid);
        corners += inc_if_interior(same_type_sides, BOTTOM_RIGHT, c, xi as i32 +1, yi as i32+1, &grid);
        corners += inc_if_interior(same_type_sides, TOP_RIGHT, c, xi as i32+1, yi as i32-1, &grid);

        // println!("({c}@{xi}, {yi}) == {corners}");
    }

    for (x, y) in &seen {
        grid[*y][*x] = ' ';
    }
    
   
    let area = seen.len() as i32;
    let perim = corners;
    //  println!("{c}: {area} * {perim} = {}", area * perim);
    area * perim
}


pub fn part2() {
    let mut grid: Vec<Vec<_>> = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day12.txt").unwrap()
        .lines()
        .map(|row| row.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();
    let mut total_price = 0;
    for y in 0..height {
        for x in 0..width {
            total_price += get_area_price2(&mut grid, y, x, height, width);
        }
    }

    println!("Total Price: {total_price}");
}