use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut cache = HashMap::new();
    let res = contents.split(" ").map(|c| c.parse::<u128>().unwrap()).fold(0, |acc, no| acc + number_count(no, 75, &mut cache));

    println!("Result: {}", res);

    Ok(())
}

fn number_count(no: u128, splits_left: u8, cache: &mut HashMap<(u128, u8), u128>) -> u128 {
    if splits_left == 0 {
        return 1;
    }

    let cache_res = cache.get(&(no, splits_left));
    if cache_res.is_some() {
        return *cache_res.unwrap();
    }

    if no == 0 {
        let res = number_count(1, splits_left - 1, cache);
        cache.insert((no, splits_left), res);
        return res;
    }

    let no_str = no.to_string();
    if no_str.len() % 2 == 0 {
        let half = no_str.len() / 2;
        let res = number_count(no_str[..half].parse::<u128>().unwrap(), splits_left - 1, cache) + number_count(no_str[half..].parse::<u128>().unwrap(), splits_left - 1, cache);
        cache.insert((no, splits_left), res);
        return res;
    }

    let res = number_count(no * 2024, splits_left - 1, cache);
    cache.insert((no, splits_left), res);
    res
}