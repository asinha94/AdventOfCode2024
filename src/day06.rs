use std::fs::{self, DirEntry};
use std::ops::AddAssign;

const OBSTACLE: char = '#';
const VISITED_MARKER: char = 'X';


#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn from_usize(x: usize, y: usize) -> Coordinate {
        Coordinate{
            x: x as i32,
            y: y as i32
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn get_direction_vector(&self) -> Coordinate {
        match self {
            Direction::North => Coordinate{x:  0, y: -1},
            Direction::East  => Coordinate{x:  1, y:  0},
            Direction::South => Coordinate{x:  0, y:  1},
            Direction::West  => Coordinate{x: -1, y:  0}
        }
    }

    fn get_oposite_direction_vector(&self) -> Coordinate {
        let opposite = match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        };

        opposite.get_direction_vector()
    }

    fn get_clockwise_direction(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }
}

struct Guard {
    direction: Direction,
    position: Coordinate,
    map: Vec<Vec<char>>
}

impl Guard {

    fn get_bounds(&self) -> (Coordinate, Coordinate) {
        let origin = Coordinate{x: 0, y: 0};
        let far_corner = Coordinate{
            x: (self.map[0].len() - 1) as i32,
            y: (self.map.len() - 1)    as i32
        };
        (origin, far_corner)
    }

    fn hit_boundary(&self) -> bool {
        let x = self.position.x as usize;
        let y = self.position.y as usize;
        self.map[y][x] == OBSTACLE
    }

    fn step(&mut self) -> bool {

        /* Mark standing position */
        let x = self.position.x as usize;
        let y = self.position.y as usize;
        self.map[y][x] = VISITED_MARKER;

        self.position += self.direction.get_direction_vector();

        /* Check if we've exceeded our maps' bounds */
        let (origin, far_corner) = self.get_bounds();
        
        let (lower_x, lower_y) = (origin.x, origin.y);
        if self.position.x < lower_x || self.position.y < lower_y {
            return false;
        }

        let (upper_x, upper_y) = (far_corner.x, far_corner.y);
        if self.position.x > upper_x || self.position.y > upper_y {
            return false;
        }

        /* If we hit an obstacle, reverse course and turn right */
        if self.hit_boundary() {
            self.position += self.direction.get_oposite_direction_vector();
            self.direction = self.direction.get_clockwise_direction();
            return self.step();
        }

        true
    }

    fn get_path_position_count(&self) -> usize {
        self.map.iter()
            .enumerate()
            .map(|(_, row)|  row.iter()
                .enumerate()
                .filter(|(_, &c)| c == VISITED_MARKER)
                .count())
            .sum()
    }
}





pub fn part1() {
    let grid: Vec<_> = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day06.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();


    let starting_position_vec: Vec<_> = grid
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            for (x, c) in row.iter().enumerate() {
                if *c == '^' {
                    return Some(Coordinate::from_usize(x, y))
                }
            }
            None
            })
        .collect();
    
    let starting_position = starting_position_vec[0];
    let mut guard = Guard{
        direction: Direction::North,
        position: starting_position,
        map: grid
    };

    /* Move to all possible positions */
    while guard.step() {};

    let positions = guard.get_path_position_count();
    println!("All positions: {positions}");


}