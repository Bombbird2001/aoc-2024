use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut before_set = HashSet::new();
    let mut all_pairs_added = false;
    let mut correct_sum = 0;
    contents.lines().for_each(|line| {
        if line.is_empty() {
            all_pairs_added = true;
            return;
        }
        if !all_pairs_added {
            before_set.insert(line);
        } else {
            let nos = line.split(",").collect::<Vec<_>>();
            for i in (0..nos.len()).rev() {
                let curr_no = nos[i];
                for j in i + 1..nos.len() {
                    let next_no = nos[j];
                    let combined = next_no.to_owned() + "|" + curr_no;
                    if before_set.contains(combined.as_str()) {
                        return;
                    }
                }
            }

            correct_sum += nos[nos.len() / 2].parse::<i32>().unwrap();
        }
    });

    println!("Correct sum: {}", correct_sum);

    Ok(())
}
