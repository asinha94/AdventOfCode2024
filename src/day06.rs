use std::collections::HashSet;
use std::fs;
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


#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

enum TraverserStatus {
    Found,
    NotFound,
    Exited
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


struct Marker {
    marker: u8
}

impl Marker {
    /* The default in the input used for a walkable path */
    const NULL_CHAR: char = '.'; 
    const NULL_U8: u8 = Marker::NULL_CHAR as u8;

    fn create(marker: char) -> Marker{
        Marker{
            marker: marker as u8
        }
    }

    fn as_char(&self) -> char {
        self.marker as char
    }

    fn get_direction_bit(direction: Direction) -> u8 {
        match direction {
            Direction::North => 1 << 0,
            Direction::East  => 1 << 1,
            Direction::South => 1 << 2,
            Direction::West  => 1 << 3
        }
    }

    fn set_direction(&mut self, direction: Direction) {
        let bit = Marker::get_direction_bit(direction);
        let mut marker = self.marker - Marker::NULL_U8;
        marker |= bit;
        self.marker = marker + Marker::NULL_U8;
    }

    fn test_direction(&self, direction: Direction) -> bool {
        let bit = Marker::get_direction_bit(direction);
        let marker = self.marker  - Marker::NULL_U8;
        marker & bit > 0
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

        /* Mark standing position. (x,y) is always on the map */
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

    fn within_bounds(&self, position: Coordinate) -> bool {
        let (origin, far_corner) = self.get_bounds();
        let (lower_x, lower_y) = (origin.x, origin.y);
        let (upper_x, upper_y) = (far_corner.x, far_corner.y);

        /* Check if we've exceeded our maps' bounds */
        if position.x < lower_x || position.y < lower_y {
            return false;
        }
        
        if position.x > upper_x || position.y > upper_y {
            return false;
        }

        true
    }

    fn step_till_cycle(&mut self, discovered: &mut HashSet<(i32, i32)>) -> TraverserStatus {

        /* Mark standing position. (x,y) is always on the map */
        let x = self.position.x as usize;
        let y = self.position.y as usize;

        let mut marker = Marker::create(self.map[y][x]);
        marker.set_direction(self.direction);
        self.map[y][x] = marker.as_char();

        /* Check if we can turn right */
        let mut position = self.position.clone();
        let turn_right = self.direction.get_clockwise_direction();
        position += turn_right.get_direction_vector();

        if self.within_bounds(position) {
            /* Does turning right put us in a loop? */
            let coords = (position.x, position.y);
            let (m_x, m_y) = (coords.0 as usize, coords.1 as usize);

            /* Check if its a obstacle */
            let right_cell = self.map[m_y][m_x];
            if right_cell != OBSTACLE {
                let right_marker = Marker::create(right_cell);

                if right_marker.test_direction(turn_right) {
                    /* found a loop, is it unique? */
                    if !discovered.contains(&coords) {
                        discovered.insert(coords);
                        return TraverserStatus::Found;
                    }
                }
            }
            
        }

        /* Proceed onwards */
        self.position += self.direction.get_direction_vector();
        if !self.within_bounds(self.position) {
            return TraverserStatus::Exited;
        }


        /* If we hit an obstacle, reverse course and turn right */
        while self.hit_boundary() {
            self.position += self.direction.get_oposite_direction_vector();
            self.direction = self.direction.get_clockwise_direction();
            self.position += self.direction.get_direction_vector();
        }

        TraverserStatus::NotFound
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

    /* Move to all possible positions till the guard exits */
    while guard.step() {};

    let positions = guard.get_path_position_count();
    println!("All positions: {positions}");


}


pub fn part2() {
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
    let mut discovered: HashSet<(i32, i32)> = HashSet::new();

     'GuardWalk: loop {
        let mut guard = Guard{
            direction: Direction::North,
            position: starting_position,
            map: grid.clone()
        };
    
        /* Move to all possible positions till the guard exits */
        loop {
            match guard.step_till_cycle(&mut discovered) {
                TraverserStatus::Exited => break 'GuardWalk,
                TraverserStatus::Found => break,
                TraverserStatus::NotFound => (),
            }
        }

    }

    println!("All placements: {}", discovered.len());


}