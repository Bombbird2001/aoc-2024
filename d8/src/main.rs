use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut antenna_locations = HashMap::new();

    let width = contents.lines().next().unwrap().len();
    let height = contents.lines().count();

    contents.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if c == '.' {
                return;
            }
            antenna_locations.entry(c).or_insert(Vec::new()).push((row as i32, col as i32));
        });
    });

    println!("Antinode count 1: {}", antinodes_part1(&antenna_locations, width, height));
    println!("Antinode count 2: {}", antinodes_part2(&antenna_locations, width, height));

    Ok(())
}

fn antinodes_part1(antenna_locations: &HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    let mut antinodes = HashSet::new();

    antenna_locations.iter().for_each(|(_c, locations)| {
        for i in 0..locations.len() {
            for j in i + 1..locations.len() {
                let (x1, y1) = locations[i];
                let (x2, y2) = locations[j];

                let (a1x, a1y) = ((2 * x1 + x2) as f32 / 3f32, (2 * y1 + y2) as f32 / 3f32);
                let (a2x, a2y) = ((x1 + 2 * x2) as f32 / 3f32, (y1 + 2 * y2) as f32 / 3f32);
                if a1x.fract() == 0f32 && a1y.fract() == 0f32 {
                    antinodes.insert((a1x as i32, a1y as i32));
                }
                if a2x.fract() == 0f32 && a2y.fract() == 0f32 {
                    antinodes.insert((a2x as i32, a2y as i32));
                }

                let (a3x, a3y) = (2 * x1 - x2, 2 * y1 - y2);
                let (a4x, a4y) = (2 * x2 - x1, 2 * y2 - y1);
                if a3x >= 0 && a3x < width as i32 && a3y >= 0 && a3y < height as i32 {
                    antinodes.insert((a3x, a3y));
                }
                if a4x >= 0 && a4x < width as i32 && a4y >= 0 && a4y < height as i32 {
                    antinodes.insert((a4x, a4y));
                }
            }
        }
    });

    antinodes.len()
}

fn antinodes_part2(antenna_locations: &HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    let mut antinodes = HashSet::new();

    antenna_locations.iter().for_each(|(_c, locations)| {
        for i in 0..locations.len() {
            for j in i + 1..locations.len() {
                let (x1, y1) = locations[i];
                let (x2, y2) = locations[j];
                let (dx, dy) = (x2 - x1, y2 - y1);

                let mut c_x1 = x2;
                let mut c_y1 = y2;
                while c_x1 >= 0 && c_x1 < width as i32 && c_y1 >= 0 && c_y1 < height as i32 {
                    antinodes.insert((c_x1, c_y1));
                    c_x1 += dx;
                    c_y1 += dy;
                }
                c_x1 = x1;
                c_y1 = y1;
                while c_x1 >= 0 && c_x1 < width as i32 && c_y1 >= 0 && c_y1 < height as i32 {
                    antinodes.insert((c_x1, c_y1));
                    c_x1 -= dx;
                    c_y1 -= dy;
                }
            }
        }
    });

    antinodes.len()
}