use std::fs;


struct Blocks {
    block_start: usize,
    file_id: usize,
    size: usize,
    available_free_size: usize
}

impl Blocks {
    fn calculate_checksum_from_offset(&self, offset: usize, count: usize) -> usize {
        let block_start = self.block_start + offset;
        self.file_id * calculate_sum(block_start, count)
    }

    fn calculate_checksum(&self) -> usize {
        self.calculate_checksum_from_offset(0, self.size)
    }

    fn calculate_checksum_from_block(&self, other: &Blocks, offset: usize, count: usize) -> usize {
        let block_start = other.block_start + offset;
        self.file_id * calculate_sum(block_start, count)
    }
}

/* Sums the numbers from start to start + count (exclusive) */
fn calculate_sum(start: usize, count: usize) -> usize {

    let end = start + count;
    (start..end).sum()
}

fn calculate_compact_checksum(free_blocks: &Vec<Blocks>, file_blocks: &Vec<Blocks>, checksum: usize) -> usize {
    let mut checksum = checksum;
    let mut file_block_idx = file_blocks.len() - 1;
    let mut file_block_consumed = 0;
    for free_block in free_blocks {
        
        let mut free_block_used = 0;

        // Keep looping till we use up all of the block
        while free_block_used < free_block.size {
            // We might not have fully consumed the block from the end
            let file_block = &file_blocks[file_block_idx];
            let file_block_remaining = file_block.size - file_block_consumed;

            if free_block.block_start > file_block.block_start {
                return checksum;
            }

            // Check if we can use the entirety of the free_block
            let remaining_free_block = free_block.size - free_block_used;
            if remaining_free_block <= file_block_remaining {

                // Remove old checksum, add new one
                let old_block_start = file_block_remaining - remaining_free_block;
                let old = file_block.calculate_checksum_from_offset(old_block_start, remaining_free_block);
                let new =  file_block.calculate_checksum_from_block(&free_block, free_block_used, remaining_free_block);
                checksum += new;
                checksum -= old;
                println!("REM: {old}\nADD: {new}");
                
                // Check if we can also iterate the file_block pointer
                file_block_consumed += remaining_free_block;
                if file_block_consumed == file_block.size {
                    file_block_idx -= 1;
                    file_block_consumed = 0;
                }

                break;
            }

            // We get here if there is more space in the free_block than we need
            let old= file_block.calculate_checksum_from_offset(0, file_block_remaining);
            let new= file_block.calculate_checksum_from_block(&free_block, free_block_used, file_block_remaining);
            checksum -= old;
            checksum += new;
            println!("REM: {old}\nADD: {new}");

            file_block_idx -= 1;
            file_block_consumed = 0;
            free_block_used += file_block_remaining;

        }
    }
    checksum
}

pub fn part1() {
    let disk_layout = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day09.txt").unwrap();
    let mut checksum: usize = 0;
    
    let mut block_count_total: usize = 0;
    let mut free_blocks = vec![];
    let mut file_blocks = vec![];

    disk_layout.char_indices()
        .for_each(|(file_id, c)| {
            let is_file = file_id & 0x1 == 0;
            let block_size = c.to_digit(10).unwrap() as usize;

            if block_size == 0 {
                return;
            }

            let block = Blocks{
                block_start: block_count_total,
                file_id: if is_file {file_id / 2} else {0},
                size: block_size,
                available_free_size: block_size, /* unused in part1 */
            };

            if is_file {
                println!("ADD: {}", block.calculate_checksum());
            }
            
            checksum += if is_file {block.calculate_checksum()} else {0};
            block_count_total += block_size;

            let block_list = if is_file {&mut file_blocks} else {&mut free_blocks};
            block_list.push(block);
            
        });


        let compact_checksum = calculate_compact_checksum(&free_blocks, &file_blocks, checksum);
        println!("Checksum: {compact_checksum}");

}

fn calculate_compact_checksum2(free_blocks: &mut Vec<Blocks>, file_blocks: &Vec<Blocks>, checksum: usize) -> usize {
    let mut checksum = checksum;


    for file_block in file_blocks.iter().rev() {

        /*
         * For each block from the end
         * till either we reach the end or there are no empty free spaces in front of it
         * Check the first free space available and keep iterting up
         */

        for free_block in &mut *free_blocks {

            if free_block.block_start > file_block.block_start {
                break;
            }

            if free_block.available_free_size >= file_block.size {

                let offset = free_block.size - free_block.available_free_size;
                /* recalculate checksum */
                let old= file_block.calculate_checksum();
                let new= file_block.calculate_checksum_from_block(free_block, offset, file_block.size);

                checksum -= old;
                checksum += new;

                free_block.available_free_size -= file_block.size;
                break;
            }
        }
        
        
    }
    checksum
}


pub fn part2() {
    let disk_layout = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day09.txt").unwrap();
    let mut checksum: usize = 0;
    
    let mut block_count_total: usize = 0;
    let mut free_blocks = vec![];
    let mut file_blocks = vec![];

    disk_layout.char_indices()
        .for_each(|(file_id, c)| {
            let is_file = file_id & 0x1 == 0;
            let block_size = c.to_digit(10).unwrap() as usize;

            if block_size == 0 {
                return;
            }

            let block = Blocks{
                block_start: block_count_total,
                file_id: if is_file {file_id / 2} else {0},
                size: block_size,
                available_free_size: block_size,
            };

            if is_file {
                println!("ADD: {}", block.calculate_checksum());
            }
            
            checksum += if is_file {block.calculate_checksum()} else {0};
            block_count_total += block_size;

            let block_list = if is_file {&mut file_blocks} else {&mut free_blocks};
            block_list.push(block);
            
        });


        let compact_checksum = calculate_compact_checksum2(&mut free_blocks, &file_blocks, checksum);
        println!("Checksum: {compact_checksum}");
}