use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut safe_count = 0;
    contents.lines().for_each(|line| {
        let mut safe = check_line(line, 0, false);
        if !safe {
            for i in 0..line.split_ascii_whitespace().count() {
                safe = check_line(line, i, true);
                if safe {
                    break;
                }
            }
        }

        if safe {
            safe_count += 1;
        }
    });
    println!("Safe count: {}", safe_count);

    Ok(())
}

fn check_line(line: &str, remove_index: usize, damp: bool) -> bool {
    let mut prev_no: Option<i32> = None;
    let mut positive: Option<bool> = None;
    let mut safe = true;
    for (_, no) in line.split_ascii_whitespace().enumerate().filter(|(i, _)| !damp || *i != remove_index) {
        let parsed = no.parse::<i32>().unwrap();
        if prev_no.is_none() {
            prev_no = Some(parsed);
            continue;
        }
        let prev_num = prev_no.unwrap();
        if (parsed - prev_num).abs() > 3 || parsed == prev_num
            || (positive.is_some() && positive.unwrap() != (parsed > prev_num)) {
            safe = false;
            break;
        }
        prev_no = Some(parsed);
        positive = Some(parsed > prev_num);
    }
    safe
}
