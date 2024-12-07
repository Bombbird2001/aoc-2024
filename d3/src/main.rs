use std::fs::File;
use std::io::{Read, Result};
use regex::Regex;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let do_re = Regex::new(r"do\(\)").unwrap();
    let do_offsets = do_re.find_iter(&contents).map(|m| m.start()).collect::<Vec<_>>();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let dont_offsets = dont_re.find_iter(&contents).map(|m| m.start()).collect::<Vec<_>>();

    let mut sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut do_index: i32 = -1;
    let mut dont_index: i32 = -1;
    re.captures_iter(&contents).for_each(|capture| {
        let match_offset = capture.get(0).unwrap().start();

        if do_offsets.len() > 0 && do_index < (do_offsets.len() - 1) as i32 && match_offset > do_offsets[(do_index + 1) as usize] {
            do_index += 1;
        }
        if dont_offsets.len() > 0 && dont_index < (dont_offsets.len() - 1) as i32 && match_offset > dont_offsets[(dont_index + 1) as usize] {
            dont_index += 1;
        }

        if dont_index > do_index {
            return;
        }

        if dont_index >= 0 && do_index >= 0 && dont_offsets[dont_index as usize] > do_offsets[do_index as usize] {
            return;
        }

        let mul1 = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let mul2 = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum += mul1 * mul2;
    });
    println!("Sum: {}", sum);

    Ok(())
}
