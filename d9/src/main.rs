use std::cmp::min;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Checksum 1: {}", checksum_part_1(contents.as_str()));

    println!("Checksum 2: {}", checksum_part_2(contents));

    Ok(())
}

fn checksum_part_1(contents: &str) -> u64 {
    let mut file_blocks = Vec::new();
    let mut empty_blocks = Vec::new();
    let mut position = 0;
    contents.chars().enumerate().for_each(|(i, c)| {
        let block_size = c.to_string().parse::<u32>().unwrap();
        if i % 2 == 0 && block_size > 0 {
            file_blocks.push((position, block_size));
        } else if block_size > 0 {
            empty_blocks.push((position, block_size));
        }
        position += block_size;
    });

    let mut checksum = 0u64;
    let mut empty_index = 0;

    file_blocks.iter_mut().enumerate().rev().for_each(|(file_id, (position, block_size))| {
        let mut position_sum = 0u32;
        while *block_size > 0 {
            if empty_index >= empty_blocks.len() || empty_blocks[empty_index].0 > *position {
                break;
            }
            if empty_blocks[empty_index].1 <= 0 {
                empty_index += 1;
                continue;
            }
            let (empty_position, empty_size) = empty_blocks[empty_index];
            let move_blocks = min(*block_size, empty_size);
            position_sum += move_blocks * (2 * empty_position + (move_blocks - 1)) / 2;
            *block_size -= move_blocks;
            empty_blocks[empty_index].1 -= move_blocks;
            empty_blocks[empty_index].0 += move_blocks;
            if empty_blocks[empty_index].1 == 0 {
                empty_index += 1;
            }
        }
        checksum += position_sum as u64 * file_id as u64;
    });

    checksum += file_blocks.iter().enumerate().filter(|(_, (_, size))| *size > 0).fold(0, |acc, (id, (position, block_size))| {
        acc + (*block_size as u64 * (2 * position + (block_size - 1)) as u64 / 2) * id as u64
    });

    checksum
}

fn checksum_part_2(contents: String) -> u64 {
    let mut checksum = 0u64;

    let mut position = 0;
    let mut combined_blocks = contents.chars().enumerate().map(|(index, c)| {
        let block_size = c.to_string().parse::<u32>().unwrap();
        let res = if index % 2 == 0 {
            (index as i32 / 2, position, block_size)
        } else {
            (-1, position, block_size)
        };
        position += block_size;
        res
    }).collect::<Vec<_>>();

    let mut index = combined_blocks.len() as i32 - 1;
    while index >= 0 {
        let file_to_move = combined_blocks[index as usize].0;
        let file_curr_position = combined_blocks[index as usize].1;
        if file_to_move == -1 {
            index -= 1;
            continue;
        }
        let file_size = combined_blocks[index as usize].2;

        for i in 0..combined_blocks.len() {
            let file_id = combined_blocks[i].0;
            let position = combined_blocks[i].1;
            let block_size = combined_blocks[i].2;
            if position >= file_curr_position {
                break;
            }
            if file_id != -1 {
                continue;
            }
            if file_size <= block_size {
                // println!("Start position {}, block size {}, adding {}", empty_blocks[i].0, *block_size, (*block_size as u64 * (2 * empty_blocks[i].0 + (*block_size - 1)) as u64 / 2) * file_id as u64);
                combined_blocks[i].2 -= file_size;
                combined_blocks[i].1 += file_size;
                combined_blocks.remove(index as usize);
                combined_blocks.insert(i, (file_to_move, position, file_size));
                index += 1;
                // println!("Moved file {} to position {}", file_to_move, position);
                break;
            }
        }
        index -= 1;
    }

    checksum += combined_blocks.iter().filter(|(id, _, size)| *size > 0 && *id > 0).fold(0, |acc, (id, position, block_size)| {
        // println!("Start position {}, block size {}, id {}, adding {}", position, *block_size, id, (*block_size as u64 * (2 * position + (*block_size - 1)) as u64 / 2) * id as u64);
        acc + (*block_size as u64 * (2 * position + (block_size - 1)) as u64 / 2) * *id as u64
    });

    checksum
}