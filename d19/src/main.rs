use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut towels = Vec::new();
    let mut patterns = Vec::new();

    let mut towel_mode = true;
    contents.lines().for_each(|line| {
        if line.is_empty() {
            towel_mode = false;
            return;
        }
        if towel_mode {
            line.split(", ").for_each(|towel| {
                towels.push(towel);
            });
        } else {
            patterns.push(line);
        }
    });

    let mut count = 0usize;
    let mut total = 0;
    for i in 0..patterns.len() {
        let mut cache = HashMap::new();
        let pattern = patterns[i];
        if can_match(&pattern, 0, &towels) {
            count += 1;
        }
        let res = match_count(i, 0, &patterns, &towels, &mut cache);
        // println!("Pattern: {}, Count: {}", pattern, res);
        total += res;
    }

    println!("Count: {}", count);
    println!("Total: {}", total);

    Ok(())
}

fn can_match(pattern: &str, start_index: usize, towels: &Vec<&str>) -> bool {
    if start_index >= pattern.chars().count() {
        return true;
    }

    for towel in towels {
        if pattern[start_index..].starts_with(towel) {
            if can_match(pattern, start_index + towel.chars().count(), towels) {
                return true;
            }
        }
    }

    false
}

fn match_count(pattern_index: usize, start_index: usize, patterns: &Vec<&str>, towels: &Vec<&str>, cache: &mut HashMap<usize, usize>) -> usize {
    let pattern = patterns[pattern_index];
    if start_index >= pattern.chars().count() {
        return 1;
    }

    if let Some(&count) = cache.get(&start_index) {
        return count;
    }

    let mut count = 0;
    for towel in towels {
        if pattern[start_index..].starts_with(towel) {
            count += match_count(pattern_index, start_index + towel.chars().count(), patterns, towels, cache);
        }
    }

    cache.insert(start_index, count);
    count
}