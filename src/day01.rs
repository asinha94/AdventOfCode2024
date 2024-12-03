use std::fs;
use std::collections::HashMap;


pub fn part1() {
    let input_file = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day01.txt").unwrap();

    let (mut first, mut second): (Vec<i32>, Vec<i32>) = input_file
        .lines()
        .map(|l| {
            let sp: Vec<&str> = l.split(" ").collect();
            (sp[0].parse::<i32>().unwrap(), sp[sp.len()-1].parse::<i32>().unwrap())
        }).unzip();

    first.sort();
    second.sort();

    let mut sum = 0;
    for (f,s) in first.iter().zip(second.iter()) {
        sum += f.abs_diff(*s);
    }
    
    println!("Answer: {sum}");
    
}


pub fn part2() {
    let input_file = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day01.txt").unwrap();

    let mut left = vec![];
    let mut right = HashMap::new();

    for line in input_file.lines() {
        let mut s = line.split_ascii_whitespace();
        let l: i32 = s.next().unwrap().parse().unwrap();
        let r: i32 = s.next().unwrap().parse().unwrap();
        left.push(l);
        right.entry(r).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut sum = 0;
    for num in left {
        match right.get(&num) {
            Some(val) => sum += num * val,
            None => ()
        }
    }
    
    println!("Answer: {sum}");
    
}