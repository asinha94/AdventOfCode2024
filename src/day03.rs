use std::fs;

const MATCH_PREFIX: &str = "mul(";
const DO: &str = "do()";
const DONT: &str = "don't()";
const ASCII_ZERO: u8 = b'0';


pub fn part1() {
    let input_file = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day03.txt").unwrap();
    let input_bytes = input_file.as_bytes();

    let mul_sum: i32  = input_file
        .match_indices(MATCH_PREFIX)
        .filter_map(|(idx, _)| {
            let remaining_chars = input_bytes.len() - (idx + MATCH_PREFIX.len());
            if remaining_chars < (2 + 1 + 2) { // 2 brackets, 1 comma, at least 2 digits
                return None
            }

            // Get first digit
            let mut i = idx + MATCH_PREFIX.len();
            let mut j = 0;
            let mut nums = [0, 0];
            let substr_term = [b',', b')'];
            while j < 2 {

                let mut num: i32 = 0;
                let mut digit_count = 0;
                while i < input_bytes.len() {
                    let c = input_bytes[i];
                    if c.is_ascii_digit() {
                        if digit_count == 3 {
                            // too many digits
                            return None;
                        }
                        digit_count += 1;
                        num *= 10;
                        num += (c - ASCII_ZERO) as i32;
                        i += 1;
                        continue;
                    }
    
                    // Avoids the case of the string "mul(0" being found
                    // also only ',' or ')' afterwards
                    if digit_count == 0 || c != substr_term[j] {
                        return None;
                    }

                    i += 1;
                    break;
                }

                nums[j] = num;
                j += 1;
            }
            
            println!("Got {} {}", nums[0], nums[1]);
            Some(nums[0]*nums[1])
        })     
        .sum();

    println!("{mul_sum}");
    
}


#[derive(Clone)]
enum Operations {
    MUL(i32, i32),
    DO,
    DONT
}

impl Operations {
    fn operand_match_mul(idx: usize, input: &[u8]) -> Option<Operations> {

        /* Parse the mul( */
        let mut i = idx;
        let mut k = 0;
        let op_prefix = MATCH_PREFIX.as_bytes();
        while i < input.len() {
            if k == op_prefix.len() {
                break;
            }

            if input[i] != op_prefix[k] {
                return None;
            }
            k += 1;
            i += 1;
        }

        let mut j = 0;
        let mut nums: [i32; 2] = [0, 0];
        let terms = [b',', b')'];
        while j < nums.len() {
            let mut num = 0i32;
            let mut len = 0;
            while i < input.len() {

                let c = input[i];
                i += 1;

                /* Check for at most 3 digits */
                if c.is_ascii_digit() {
                    // too long
                    if len == 3 {
                        return None;
                    }

                    num *= 10;
                    num += (c - ASCII_ZERO) as i32;
                    len += 1;
                    continue;
                }

                if len == 0 || c != terms[j] {
                    return None;
                }

                break;
            }

            nums[j] = num;
            j += 1;
        }

        Some(Operations::MUL(nums[0], nums[1]))
    }

    fn operand_match_do_or_dont(idx: usize, input: &[u8]) -> Option<Operations> {

        let mut j = 0;
        let ops = [
            (DO, Operations::DO),
            (DONT, Operations::DONT)];

        while j < ops.len() {
            let mut i = idx;
            let mut k = 0;
            let op = ops[j].0.as_bytes();
            while i < input.len() {
                
                if op[k] != input[i] {
                    break;
                }

                i += 1;
                k += 1;

                if k == op.len() {
                    return Some(ops[j].1.clone());
                }
            }
            j += 1;
        }
    
        None
    }
}


pub fn part2() {
    let input_file = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day03.txt").unwrap();

    //for (i, c) in 
    
    let ops: Vec<_> = input_file
        .char_indices()
        .filter_map(| (i, c)| match c {
            'd' => Operations::operand_match_do_or_dont(i, input_file.as_bytes()),
            'm' => Operations::operand_match_mul(i, input_file.as_bytes()),
            _ => None
    }).collect();

    let mut sum = 0;
    let mut enabled = true;
    for op in ops {
        match op {
            Operations::DO => enabled = true,
            Operations::DONT => enabled = false,
            Operations::MUL(x, y) => sum += if enabled {x*y} else {0}
        };
    }

    println!("Got Sum: {sum}");

}