use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut seq_price_sum = HashMap::new();

    let total = contents.lines().fold(0u64, |acc, line| {
        let no = line.parse::<u64>().unwrap();
        let nos = get_secret_numbers(no, 2000);
        let mut prev_price = nos[0] % 10;
        let mut seq_diff = Vec::new();
        let mut seq_seen = HashSet::new();
        for i in 1..nos.len() {
            let price = nos[i] % 10;
            seq_diff.push(price as i64 - prev_price as i64);
            if seq_diff.len() > 4 {
                seq_diff.remove(0);
            }
            if seq_diff.len() == 4 {
                let seq = (seq_diff[0], seq_diff[1], seq_diff[2], seq_diff[3]);
                if !seq_seen.contains(&seq) {
                    let sum = seq_price_sum.entry(seq).or_insert(0);
                    *sum += price;
                    seq_seen.insert(seq);
                }
            }
            prev_price = price;
        }

        acc + nos.last().unwrap()
    });
    println!("Total: {}", total);

    let max_price_sum = seq_price_sum.iter().max_by_key(|(_k, &v)| v).unwrap();
    println!("Max price sum: {:?}", max_price_sum);

    Ok(())
}

fn get_secret_numbers(start: u64, cycles: u32) -> Vec<u64> {
    let mut numbers = Vec::new();
    let mut current = start;
    for _ in 0..cycles {
        current = prune(mix(current * 64, current));
        current = prune(mix(current / 32, current));
        current = prune(mix(current * 2048, current));
        numbers.push(current);
    }
    numbers
}

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn prune(value: u64) -> u64 {
    value % 16777216
}