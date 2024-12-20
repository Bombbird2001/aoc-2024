use std::collections::{VecDeque};
use std::fs::File;
use std::io::{Read, Result};
use std::mem;

const CHEAT_DIST: i32 = 20;
const SPEED_UP_THRESHOLD: i32 = 100;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut map = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;
    contents.lines().enumerate().for_each(|(row, line)| {
        map.push(line.chars().collect::<Vec<char>>());
        line.chars().enumerate().for_each(|(col, c)| {
            if c == 'S' {
                start_x = col;
                start_y = row;
            } else if c == 'E' {
                end_x = col;
                end_y = row;
            }
        });
    });

    let no_cheats_dist = bfs(start_x, start_y, end_x, end_y, &map, None).unwrap();
    println!("Distance without cheats: {}", no_cheats_dist);

    let start_distances = single_source_distance(start_x, start_y, &map, None);
    let end_distances = single_source_distance(end_x, end_y, &map, None);

    let mut more_than_threshold = 0;
    for x1 in 0..map[0].len() as i32 {
        for y1 in 0..map.len() as i32 {
            // println!("Checking ({}, {})", x1, y1);

            if x1 < 0 || x1 >= map[0].len() as i32 || y1 < 0 || y1 >= map.len() as i32 {
                continue;
            }

            if map[y1 as usize][x1 as usize] == '#' {
                continue;
            }

            for x2 in x1 - CHEAT_DIST..x1 + CHEAT_DIST + 1 {
                let x_offset = (x2 - x1).abs();
                for y2 in y1 - CHEAT_DIST + x_offset..y1 + CHEAT_DIST - x_offset + 1 {
                    if x2 < 0 || x2 >= map[0].len() as i32 || y2 < 0 || y2 >= map.len() as i32 {
                        continue;
                    }
                    if (x1, y1) == (x2, y2) {
                        continue;
                    }
                    if map[y2 as usize][x2 as usize] == '#' {
                        continue;
                    }
                    let dist = x_offset + (y2 - y1).abs();

                    let x1 = x1 as usize;
                    let y1 = y1 as usize;
                    let x2 = x2 as usize;
                    let y2 = y2 as usize;


                    if start_distances[y1][x1].is_none() || end_distances[y2][x2].is_none() {
                        continue;
                    }

                    let total = start_distances[y1][x1].unwrap() + end_distances[y2][x2].unwrap() + dist;

                    if total <= no_cheats_dist - SPEED_UP_THRESHOLD {
                        more_than_threshold += 1;
                    }
                }
            }
        }
    }

    println!(">= {} steps saved: {}", SPEED_UP_THRESHOLD, more_than_threshold);

    Ok(())
}

fn bfs(start_x: usize, start_y: usize, end_x: usize, end_y: usize, map: &Vec<Vec<char>>, max_steps: Option<i32>) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut pending_queue = VecDeque::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut steps = 0;

    queue.push_back((start_x, start_y));
    visited[start_y][start_x] = true;

    while !queue.is_empty() {
        if max_steps.is_some() && steps > max_steps.unwrap() {
            return None;
        }

        let (x, y) = queue.pop_front().unwrap();

        if (x, y) == (end_x, end_y) {
            return Some(steps);
        }

        if x > 0 && !visited[y][x - 1] && map[y][x - 1] != '#' {
            pending_queue.push_back((x - 1, y));
            visited[y][x - 1] = true;
        }

        if x < map[0].len() - 1 && !visited[y][x + 1] && map[y][x + 1] != '#' {
            pending_queue.push_back((x + 1, y));
            visited[y][x + 1] = true;
        }

        if y > 0 && !visited[y - 1][x] && map[y - 1][x] != '#' {
            pending_queue.push_back((x, y - 1));
            visited[y - 1][x] = true;
        }

        if y < map.len() - 1 && !visited[y + 1][x] && map[y + 1][x] != '#' {
            pending_queue.push_back((x, y + 1));
            visited[y + 1][x] = true;
        }

        if queue.is_empty() {
            mem::swap(&mut queue, &mut pending_queue);
            steps += 1;
        }
    }

    None
}

fn single_source_distance(start_x: usize, start_y: usize, map: &Vec<Vec<char>>, max_dist: Option<i32>) -> Vec<Vec<Option<i32>>> {
    let mut distances = vec![vec![None; map[0].len()]; map.len()];

    let mut queue = VecDeque::new();
    let mut pending_queue = VecDeque::new();
    let mut steps = 0;

    queue.push_back((start_x, start_y));
    distances[start_y][start_x] = Some(0);

    while !queue.is_empty() {
        if max_dist.is_some() && steps >= max_dist.unwrap() {
            break;
        }

        let (x, y) = queue.pop_front().unwrap();

        if x > 0 && distances[y][x - 1].is_none() && map[y][x - 1] != '#' {
            pending_queue.push_back((x - 1, y));
            distances[y][x - 1] = Some(steps + 1);
        }

        if x < map[0].len() - 1 && distances[y][x + 1].is_none() && map[y][x + 1] != '#' {
            pending_queue.push_back((x + 1, y));
            distances[y][x + 1] = Some(steps + 1);
        }

        if y > 0 && distances[y - 1][x].is_none() && map[y - 1][x] != '#' {
            pending_queue.push_back((x, y - 1));
            distances[y - 1][x] = Some(steps + 1);
        }

        if y < map.len() - 1 && distances[y + 1][x].is_none() && map[y + 1][x] != '#' {
            pending_queue.push_back((x, y + 1));
            distances[y + 1][x] = Some(steps + 1);
        }

        if queue.is_empty() {
            mem::swap(&mut queue, &mut pending_queue);
            steps += 1;
        }
    }

    distances
}
