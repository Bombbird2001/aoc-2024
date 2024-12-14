use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let target_sum = contents.lines().fold(0, |acc: i64, line| {
        let colon_split = line.split(":").collect::<Vec<_>>();
        let target = colon_split[0].parse::<i64>().unwrap();
        let numbers = colon_split[1].trim().split(" ").map(|num| num.parse::<i64>().unwrap()).collect::<Vec<_>>();

        // return if can_attain_value_part_1(target, &numbers, (numbers.len() - 1) as i64) {
        //     acc + target
        // } else {
        //     acc
        // }

        return if can_attain_value_part_2(target, &numbers, (numbers.len() - 1) as i64) {
            acc + target
        } else {
            acc
        }
    });

    println!("Sum: {}", target_sum);

    Ok(())
}

fn can_attain_value_part_1(value: i64, numbers: &Vec<i64>, index: i64) -> bool {
    if value == 0 && index == -1 {
        return true;
    } else if value <= -1 {
        return false;
    } else if index == -1 {
        return false;
    }

    let minus_remaining = value - numbers[index as usize];
    if can_attain_value_part_1(minus_remaining, numbers, index - 1) {
        return true;
    }
    let divide_remaining = value / numbers[index as usize];
    if divide_remaining * numbers[index as usize] != value {
        return false;
    }

    can_attain_value_part_1(divide_remaining, numbers, index - 1)
}

fn can_attain_value_part_2(value: i64, numbers: &Vec<i64>, index: i64) -> bool {
    if value == 0 && index == -1 {
        return true;
    } else if value <= -1 {
        return false;
    } else if index == -1 {
        return false;
    }

    let minus_remaining = value - numbers[index as usize];
    if can_attain_value_part_2(minus_remaining, numbers, index - 1) {
        return true;
    }
    let divide_remaining = value / numbers[index as usize];
    if divide_remaining * numbers[index as usize] == value && can_attain_value_part_2(divide_remaining, numbers, index - 1) {
        return true;
    }

    // println!("Checking suffix: {} for value: {}", numbers[index as usize], value);
    let remaining_str = value.to_string();
    let suffix_str = numbers[index as usize].to_string();
    if !remaining_str.ends_with(suffix_str.as_str()) {
        return false;
    }

    let remaining_value = (value - numbers[index as usize]) / i64::pow(10, suffix_str.len() as u32);
    // println!("Remaining value: {}", remaining_value);

    can_attain_value_part_2(remaining_value, numbers, index - 1)
}