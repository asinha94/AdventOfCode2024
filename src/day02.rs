use std::fs;


fn report_is_safe(report: &Vec<i32>) -> bool {
    let is_increasing = (report[1] - report[0]).is_positive();

    for i in 0..report.len()-1 {
        let diff = report[i+1] - report[i];
        if is_increasing && diff.is_negative(){
            return false;
        }

        if !is_increasing && diff.is_positive() {
            return false;
        }

        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }

    };

    true
}

pub fn part1() {
    let input_file = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day02.txt").unwrap();

    let reports = input_file
        .lines()
        .filter(|r| 
            report_is_safe(&r
                .split_ascii_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect())
        )
        .count();

    println!("Reports: {reports}");
}

fn report_is_safe_level_remover(report: &Vec<i32>, start: usize, end: usize, is_rev: bool) -> bool {

    let mut has_level_been_skipped = false;
    let mut reevaluae_increasing = true;
    let mut is_increasing = true;

    let step: i32 = if is_rev {-1} else {1};
    let mut s0 = report[start];
    
    // Can't get the range syntax to work
    // Start and s1 for the iteration
    let loop_start = (start as i32) + step;;
    let mut i = loop_start;
    let end_i32 = end as i32;
    loop {

        if is_rev {
            if i < end_i32 {
                break;
            }
        } else {
            if i >= end_i32 {
                break;
            }
        }

        let s1 = report[i as usize];
        let diff = s1 - s0;

        if reevaluae_increasing {
            is_increasing = diff.is_positive();
            reevaluae_increasing = false;
        }

        if is_increasing && diff.is_negative(){
            if has_level_been_skipped {
                return false;
            }
            
            has_level_been_skipped = true;
            if i == loop_start {
                reevaluae_increasing = true;
            }
            i += step;
            continue;
        }

        if !is_increasing && diff.is_positive() {
            if has_level_been_skipped {
                return false;
            }
            
            has_level_been_skipped = true;
            if i == loop_start {
                reevaluae_increasing = true;
            }
            i += step;
            continue;
        }

        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            if has_level_been_skipped {
                return false;
            }
            
            has_level_been_skipped = true;
            if i == loop_start {
                reevaluae_increasing = true;
            }
            i += step;
            continue;
        }

        // Only update if the level doesn't fail the report
        // otherwise we're removing s1 and retring with s0 and s2
        s0 = s1;
        i += step;

    };

    true
}


fn report_is_safe_part2(report: &Vec<i32>) -> bool {
    report_is_safe(report)
        || report_is_safe_level_remover(report, 0, report.len(), false)
        || report_is_safe_level_remover(report, report.len()-1, 0, true)
}


pub fn part2() {
    let input_file = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day02.txt").unwrap();

    let reports = input_file
        .lines()
        .filter(|r| 
            report_is_safe_part2(&r
                .split_ascii_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect())
        )
        .count();

    println!("Reports: {reports}");
}