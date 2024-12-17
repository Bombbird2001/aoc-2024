use std::fs::File;
use std::io::{Read, Result};
use regex::Regex;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let width = 101;
    let height = 103;
    let center_x = width / 2;
    let center_y = height / 2;

    // 4-element vector for quadrant counts
    let mut quadrants: Vec<usize> = Vec::new();
    quadrants.resize(4, 0);

    let mut robots = Vec::new();

    contents.lines().for_each(|line| {
        let captures = robot_regex.captures(line).unwrap();
        let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let vx = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vy = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
        robots.push((x, y, vx, vy));

        let mut final_x = (x + 100 * vx) % width;
        let mut final_y = (y + 100 * vy) % height;
        if final_x < 0 {
            final_x += width;
        }
        if final_y < 0 {
            final_y += height;
        }

        // println!("{} {}", final_x, final_y);

        let quadrant: usize = match (final_x, final_y) {
            (x, y) if x > center_x && y < center_y => 0,
            (x, y) if x < center_x && y < center_y => 1,
            (x, y) if x < center_x && y > center_y => 2,
            (x, y) if x > center_x && y > center_y => 3,
            _ => return
        };

        quadrants[quadrant] += 1;
    });

    let safety_factor = quadrants.iter().fold(1, |acc, x| acc * x);

    println!("Safety factor: {}", safety_factor);

    let tree_seconds = simulate(&mut robots, width, height);
    println!("Seconds to see tree: {}", tree_seconds);

    Ok(())
}

fn simulate(robots: &mut Vec<(i32, i32, i32, i32)>, width: i32, height: i32) -> usize {
    let mut seconds = 0;
    loop {
        seconds += 1;
        if seconds % 100 == 0 {
            println!("Seconds: {}", seconds);
        }
        let mut map = vec![vec![false; width as usize]; height as usize];

        robots.iter_mut().for_each(|robot| {
            robot.0 += robot.2;
            robot.0 %= width;
            if robot.0 < 0 {
                robot.0 += width;
            }
            robot.1 += robot.3;
            robot.1 %= height;
            if robot.1 < 0 {
                robot.1 += height;
            }

            map[robot.1 as usize][robot.0 as usize] = true;
        });

        // Check rows of at least 15 consecutive true values
        for row in map.iter() {
            let mut count = 0;
            for cell in row.iter() {
                if *cell {
                    count += 1;
                } else {
                    count = 0;
                }

                if count >= 15 {
                    return seconds;
                }
            }
        }
    }
}