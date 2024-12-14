use std::fs::File;
use std::io::{Read, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut map = Vec::new();

    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;
    let mut dir_x = 0;
    let mut dir_y = 0;
    contents.lines().enumerate().for_each(|(index, line)| {
        let elements = line.split("").collect::<Vec<_>>();
        if dir_x == 0 && dir_y == 0 {
            for i in 0..elements.len() {
                let col = i as i32;
                let row = index as i32;
                if elements[i] == ">" {
                    dir_x = 1;
                    pos_x = col;
                    pos_y = row;
                    break;
                } else if elements[i] == "<" {
                    dir_x = -1;
                    pos_x = col;
                    pos_y = row;
                    break;
                } else if elements[i] == "^" {
                    dir_y = -1;
                    pos_x = col;
                    pos_y = row;
                    break;
                } else if elements[i] == "v" {
                    dir_y = 1;
                    pos_x = col;
                    pos_y = row;
                    break;
                }
            }
        }
        map.push(elements);
    });

    let initial_pos_x = pos_x;
    let initial_pos_y = pos_y;
    let initial_dir_x = dir_x;
    let initial_dir_y = dir_y;

    let mut unique_tiles = 1;

    while pos_x + dir_x >= 0 && pos_x + dir_x < map[0].len() as i32 && pos_y + dir_y >= 0 && pos_y + dir_y < map.len() as i32 {
        let next_x = pos_x + dir_x;
        let next_y = pos_y + dir_y;
        if map[next_y as usize][next_x as usize] == "#" {
            let tmp_x = dir_x;
            dir_x = -dir_y;
            dir_y = tmp_x;
            continue;
        } else if map[next_y as usize][next_x as usize] == "." {
            unique_tiles += 1;
            map[next_y as usize][next_x as usize] = "X";
        }
        pos_x = next_x;
        pos_y = next_y;
    }

    println!("Unique tiles: {}", unique_tiles);

    // Reset map
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == "X" {
                map[i][j] = ".";
            }
        }
    }

    let mut positions = 0;

    // Brute force search lmao
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != "." {
                continue;
            }

            map[i][j] = "#";

            let mut obstacle_dir_set = HashSet::new();

            pos_x = initial_pos_x;
            pos_y = initial_pos_y;
            dir_x = initial_dir_x;
            dir_y = initial_dir_y;

            while pos_x + dir_x >= 0 && pos_x + dir_x < map[0].len() as i32 && pos_y + dir_y >= 0 && pos_y + dir_y < map.len() as i32 {
                let next_x = pos_x + dir_x;
                let next_y = pos_y + dir_y;
                if map[next_y as usize][next_x as usize] == "#" {
                    if obstacle_dir_set.contains(&(dir_x, dir_y, next_x, next_y)) {
                        // Loop found, suitable position
                        positions += 1;
                        break
                    }
                    obstacle_dir_set.insert((dir_x, dir_y, next_x, next_y));
                    let tmp_x = dir_x;
                    dir_x = -dir_y;
                    dir_y = tmp_x;
                    continue;
                }
                pos_x = next_x;
                pos_y = next_y;
            }

            // Reset the map
            map[i][j] = ".";
        }
    }

    println!("Suitable positions: {}", positions);

    Ok(())
}