use std::fs;

fn parse_button_press(line: &str) -> (i64, i64) {
    let x_i = line.find("X+").unwrap() + 2;
    let comma = line.find(",").unwrap();
    let y_i = line.find("Y+").unwrap() + 2;

    let x = line.get(x_i..comma).unwrap().parse().unwrap();
    let y = line.get(y_i..).unwrap().parse().unwrap();
    (x, y)
}

fn parse_prize_location(line: &str) -> (i64, i64) {
    
    let x_i = line.find("X=").unwrap() + 2;
    let comma = line.find(",").unwrap();
    let y_i = line.find("Y=").unwrap() + 2;
    
    let x = line.get(x_i..comma).unwrap().parse().unwrap();
    let y = line.get(y_i..).unwrap().parse().unwrap();
    (x, y)
}

struct LinearEquation {
    a: i64,
    b: i64,
    eq: i64,
}

impl LinearEquation {
     fn satisfies(&self, a: i64, b: i64) -> bool {
        self.a*a + self.b*b == self.eq
     }
}

struct Equation {
    eq_x: LinearEquation,
    eq_y: LinearEquation
}

impl Equation {

    /* Solve for linear equations
     * (1) N*a + M*b = X
     * (2) K*a + L*b = Y
     * K*(1) == b(LN-MK) = NY - KX
     * b = (NY-KX) / (LN-MK)
     */
    fn calculate_b(&self) -> Option<i64> {
        let num = (self.eq_y.eq * self.eq_x.a) - (self.eq_x.eq * self.eq_y.a);
        let denom = (self.eq_y.b * self.eq_x.a) - (self.eq_x.b * self.eq_y.a);

        let b = num / denom;
        let rem = num % denom;
        (rem == 0 && b >= 0).then_some(b)
    }

    /* Solve for a given b
     * (1) N*a + M*b = X
     * (2) K*a + L*b = Y
     * a = (Y-Lb) / K
     */
    fn get_roots(&self) -> Option<(i64, i64)> {
        match self.calculate_b() {
            None => None,
            Some(b) => {
                let num = self.eq_y.eq - (self.eq_y.b * b);
                let denom = self.eq_y.a;
                let a = num / denom;
                let rem = num % denom;

                match rem == 0 && a >= 0 {
                    false => None,
                    true => {
                        (self.eq_x.satisfies(a, b) && self.eq_y.satisfies(a, b)).then_some((a,b))
                    }
                }
            }
        }
    }
}

pub fn part1() {
    let file = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day13.txt").unwrap();
    let grid: Vec<&str> = file.lines().filter(|line| !line.is_empty()).collect();

    let eqns: Vec<_> = (0..grid.len() / 3).map(|i| {
        let line = i * 3;
        let a = parse_button_press(grid[line]);
        let b = parse_button_press(grid[line+1]);
        let s = parse_prize_location(grid[line+2]);

        Equation {
            eq_x: LinearEquation{a: a.0, b: b.0, eq: s.0},
            eq_y: LinearEquation{a: a.1, b: b.1, eq: s.1},
        }
    }).collect();

    
    let mut tokens = 0;
    for (i, eq) in eqns.iter().enumerate() {
        if let Some((a, b)) = eq.get_roots() {
            tokens += 3*a + b ;
            // println!("Line {}: Satisfied with {a} a and {b} b", (i*4)+1);
        }
    }

    println!("Tokens: {tokens}");
}



pub fn part2() {
    let file = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day13.txt").unwrap();
    let grid: Vec<&str> = file.lines().filter(|line| !line.is_empty()).collect();

    let eqns: Vec<_> = (0..grid.len() / 3).map(|i| {
        let line = i * 3;
        let a = parse_button_press(grid[line]);
        let b = parse_button_press(grid[line+1]);
        let s = parse_prize_location(grid[line+2]);

        Equation {
            eq_x: LinearEquation{a: a.0, b: b.0, eq: s.0 + 10000000000000},
            eq_y: LinearEquation{a: a.1, b: b.1, eq: s.1 + 10000000000000},
        }
    }).collect();

    
    let mut tokens = 0;
    for (i, eq) in eqns.iter().enumerate() {
        if let Some((a, b)) = eq.get_roots() {
            tokens += 3*a + b ;
            // println!("Line {}: Satisfied with {a} a and {b} b", (i*4)+1);
        }
    }

    println!("Tokens: {tokens}");
}