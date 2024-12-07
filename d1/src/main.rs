use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.lines().count();
    let mut list1 = Vec::with_capacity(lines);
    let mut list2 = Vec::with_capacity(lines);
    for (index, no) in contents.split_ascii_whitespace().enumerate() {
        let parsed = no.parse::<i32>().unwrap();
        if index % 2 == 0 {
            list1.push(parsed);
        } else {
            list2.push(parsed);
        }
    }
    part1(list1.clone(), list2.clone());
    part2(&list1, &list2);
    Ok(())
}

fn part1(mut list1: Vec<i32>, mut list2: Vec<i32>) {
    list1.sort();
    list2.sort();
    let mut sum = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        sum += (a - b).abs();
    }
    println!("Part 1 answer: {}", sum);
}

fn part2(list1: &Vec<i32>, list2: &Vec<i32>) {
    let mut no_count = HashMap::new();
    for no in list2 {
        let count = no_count.entry(*no).or_insert(0);
        *count += 1;
    }
    let mut sum = 0;
    for no in list1 {
        let count = *(no_count.get(no).unwrap_or(&0));
        sum += no * count;
    }
    println!("Part 2 answer: {}", sum);
}
