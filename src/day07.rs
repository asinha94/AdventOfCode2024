use std::{collections::HashSet, fs};

fn calibration_correct(total: i64, operators: &Vec<i64>, use_concat: bool) -> bool {
    let mut all_possible_totals = HashSet::new();
    all_possible_totals.insert(operators[0]);
    
    for (i, operator) in operators.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let mut totals = vec![];
        for possible_total in &all_possible_totals {
            totals.push(operator * possible_total);
            totals.push(operator + possible_total);

            if use_concat {
                let pow10_shift = operator.to_string().len() as u32;
                let new_total = possible_total * 10_i64.pow(pow10_shift);
                totals.push(new_total + operator);
            }
        }

        all_possible_totals.clear();

        for total in totals {
            all_possible_totals.insert(total);
        }
        
    }

    all_possible_totals.contains(&total)
}


pub fn part1() {
    let operations: Vec<_> = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day07.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let l: Vec<_> = line.split(':').collect();
            let total: i64 = l[0].parse().unwrap();
            let operators: Vec<_> = l[1].split_ascii_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            (total, operators)
        }).collect();

        let sum: i64 = operations.iter()
            .filter_map(|(total, operators)| 
                calibration_correct(*total, operators, false).then_some(total))
            .sum();

        let concat_sum: i64 = operations.iter()
            .filter_map(|(total, operators)| 
                calibration_correct(*total, operators, true).then_some(total))
            .sum();

        println!("Part1 Sum: {sum}\nPart2 Sum: {concat_sum}");
}