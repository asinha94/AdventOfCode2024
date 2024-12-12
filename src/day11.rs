use std::{collections::HashMap, convert, fs, iter::{once, Chain, Once}, time::Instant};

enum Stone {
    Single(i64),
    Multiple(i64, i64)
}

impl Stone {
    fn convert(&self) -> Chain<Once<Option<i64>>, Once<Option<i64>>> {
        match self {
            Stone::Single(x) => once(Some(*x)).chain(once(None)),
            Stone::Multiple(x, y) => once(Some(*x)).chain(once(Some(*y)))
        }
    }

    fn create_multiple_stones(n: i64) -> Stone {
        let digits = n.checked_ilog10().unwrap_or(0) + 1;
        let div = 10_i64.pow(digits / 2);
        let n1 = n / div;
        let n2 = n % div;
        Stone::Multiple(n1, n2)
    }
}

fn is_even_digits(n: i64) -> bool {
    let digits = n.checked_ilog10().unwrap_or(0) + 1;
    digits & 0x1 == 0
}


pub fn part1() {
    let stones: Vec<i64> = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day11.txt").unwrap()
        .split_ascii_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();

    let mut stones_count = HashMap::new();
    for val in stones {
        stones_count.entry(val).and_modify(|e| *e += 1).or_insert(1 as usize);
    }

    for i in 0..75 {
        let start = Instant::now();

        let mut blinked = HashMap::new();
        for (stone_val, count) in stones_count {
            let stone = match is_even_digits(stone_val) {
                true => Stone::create_multiple_stones(stone_val),
                _ => match stone_val {
                    0 => Stone::Single(1),
                    _ => Stone::Single(stone_val*2024),
            }};

            match stone {
                Stone::Single(val) => blinked.entry(val)
                    .and_modify(|e| *e += count)
                    .or_insert(count),
                Stone::Multiple(val1, val2) => {
                    blinked.entry(val1)
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                    blinked.entry(val2)
                        .and_modify(|e| *e += count)
                        .or_insert(count)
                },
            };
        }

        stones_count = blinked;

        println!("loop {} took {:2?}", i+1, start.elapsed());
       
    }

    let count: usize = stones_count.iter().map(|(_, count)| count).sum();
    println!("Count: {count}");
    

    //println!("{:?}", stones);
}