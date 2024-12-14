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
    let mut incorrect_lines = Vec::new();
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
                        incorrect_lines.push(line);
                        return;
                    }
                }
            }

            correct_sum += nos[nos.len() / 2].parse::<i32>().unwrap();
        }
    });

    println!("Correct sum: {}", correct_sum);

    let mut corrected_sum = 0;
    incorrect_lines.iter().for_each(|line| {
        // Count number of elements, then add only the element with exactly floor(n/2) entries before/after it
        let nos = line.split(",").collect::<Vec<_>>();
        let length = nos.len();
        nos.iter().for_each(|no| {
            let mut count: usize = 0;
            nos.iter().for_each(|n| {
                let combined = (*n).to_owned() + "|" + no;
                if before_set.contains(combined.as_str()) {
                    count += 1;
                }
            });
            if count == length / 2 {
                corrected_sum += no.parse::<i32>().unwrap();
            }
        });
    });

    println!("Corrected sum: {}", corrected_sum);

    Ok(())
}
