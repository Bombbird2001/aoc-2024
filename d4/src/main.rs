use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let string_vec = contents.lines().collect::<Vec<_>>();
    let rows = string_vec.len();
    let cols = string_vec[0].chars().count();
    let mut total_count = 0;

    // Count rows
    for row_i in 0..rows {
        total_count += count_xmas_from(&string_vec, 0, row_i, 1, 0, rows, cols);
        total_count += count_xmas_from(&string_vec, cols - 1, row_i, -1, 0, rows, cols);
    }

    // Count columns
    for col_i in 0..cols {
        total_count += count_xmas_from(&string_vec, col_i, 0, 0, 1, rows, cols);
        total_count += count_xmas_from(&string_vec, col_i, rows - 1, 0, -1, rows, cols);
    }

    // Count diagonals
    for row_i in 0..rows {
        total_count += count_xmas_from(&string_vec, 0, row_i, 1, 1, rows, cols);
        total_count += count_xmas_from(&string_vec, cols - 1, row_i, -1, 1, rows, cols);
        total_count += count_xmas_from(&string_vec, 0, row_i, 1, -1, rows, cols);
        total_count += count_xmas_from(&string_vec, cols - 1, row_i, -1, -1, rows, cols);
    }
    for col_i in 1..cols - 1 {
        total_count += count_xmas_from(&string_vec, col_i, 0, 1, 1, rows, cols);
        total_count += count_xmas_from(&string_vec, col_i, rows - 1, 1, -1, rows, cols);
        total_count += count_xmas_from(&string_vec, col_i, 0, -1, 1, rows, cols);
        total_count += count_xmas_from(&string_vec, col_i, rows - 1, -1, -1, rows, cols);
    }

    println!("Total count: {}", total_count);

    let mut total_count = 0;

    for row_i in 1..rows - 1 {
        for col_i in 1..cols - 1 {
            total_count += is_crossing_mas(&string_vec, col_i, row_i);
        }
    }

    println!("Total count 2: {}", total_count);

    Ok(())
}

fn count_xmas_from(vec: &Vec<&str>, start_x: usize, start_y: usize, delta_x: i32, delta_y: i32, rows: usize, cols: usize) -> i32 {
    let mut x_pos = start_x as i32;
    let mut y_pos = start_y as i32;
    let mut count = 0;

    let letters = ['X', 'M', 'A', 'S'];
    let mut matched_index = 0;

    while x_pos >= 0 && (x_pos as usize) < cols && y_pos >= 0 && (y_pos as usize) < rows {
        if vec[y_pos as usize].chars().nth(x_pos as usize).unwrap() == letters[matched_index] {
            matched_index += 1;
            if matched_index == 4 {
                count += 1;
                matched_index = 0;
            }
        } else if matched_index > 0 {
            matched_index = 0;
            continue;
        }
        x_pos += delta_x;
        y_pos += delta_y;
    }

    count
}

fn is_crossing_mas(vec: &Vec<&str>, center_x: usize, center_y: usize) -> i32 {
    if vec[center_y].chars().nth(center_x).unwrap() != 'A' {
        return 0;
    }

    if !(vec[center_y - 1].chars().nth(center_x - 1).unwrap() == 'M' &&
         vec[center_y + 1].chars().nth(center_x + 1).unwrap() == 'S') &&
       !(vec[center_y - 1].chars().nth(center_x - 1).unwrap() == 'S' &&
         vec[center_y + 1].chars().nth(center_x + 1).unwrap() == 'M') {
        return 0;
    }

    if !(vec[center_y - 1].chars().nth(center_x + 1).unwrap() == 'M' &&
         vec[center_y + 1].chars().nth(center_x - 1).unwrap() == 'S') &&
       !(vec[center_y - 1].chars().nth(center_x + 1).unwrap() == 'S' &&
         vec[center_y + 1].chars().nth(center_x - 1).unwrap() == 'M') {
        return 0;
    }

    1
}
