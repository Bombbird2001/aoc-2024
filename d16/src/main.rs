mod min_heap;

use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Result};

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut map = Vec::new();
    let mut node_dist = HashMap::new();
    let mut inbound_connections = HashMap::new();
    contents.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if c == '#' {
                return;
            }
            let dist = match c {
                'E' => 0,
                _ => usize::MAX - 1000
            };
            node_dist.insert((col, row, UP), dist);
            node_dist.insert((col, row, RIGHT), dist);
            node_dist.insert((col, row, DOWN), dist);
            node_dist.insert((col, row, LEFT), dist);
        });
        map.push(line);
    });

    let (start_x, start_y);

    loop {
        // Get key with minimum value in node_dist - O(n) time since I'm too lazy to implement a min heap with augmented operations :P
        let node_dist_copy = node_dist.clone();
        // println!("Node dist len: {}", node_dist.len());
        let min_item = node_dist_copy.iter().min_by(|(_pos1, dist1), (_pos2, dist2)| dist1.cmp(dist2)).unwrap();
        let (x, y, dir) = *min_item.0;
        let dist = min_item.1;
        if map[y].chars().nth(x).unwrap() == 'S' && dir == RIGHT {
            start_x = x;
            start_y = y;
            println!("Min score: {}", dist);
            break;
        }
        for i in 0usize..4 {
            let turns = min(i.abs_diff(dir), 4 - i.abs_diff(dir));
            if turns != 1 {
                continue;
            }
            if !node_dist.contains_key(&(x, y, i)) {
                continue;
            }
            let new_dist = dist + 1000;
            let curr_entry = node_dist.entry((x, y, i)).or_insert(usize::MAX);
            if new_dist <= *curr_entry {
                if new_dist == *curr_entry {
                    inbound_connections.entry((x, y, i)).or_insert(Vec::new()).push((x, y, dir));
                } else {
                    let mut new_set = Vec::new();
                    new_set.push((x, y, dir));
                    inbound_connections.insert((x, y, i), new_set);
                    *curr_entry = new_dist;
                }
                // println!("Updated: ({}, {}, {}): {}", x, y, i, new_dist);
            }
        }

        let new_dist = min_item.1 + 1;
        let (mut new_x, mut new_y, dir) = (min_item.0.0, min_item.0.1, min_item.0.2);
        match dir {
            UP => new_y -= 1,
            RIGHT => new_x += 1,
            DOWN => new_y += 1,
            LEFT => new_x -= 1,
            _ => unreachable!()
        }
        if map[new_y].chars().nth(new_x).unwrap() != '#' && node_dist.contains_key(&(new_x, new_y, dir)) {
            let curr_entry = node_dist.entry((new_x, new_y, dir)).or_insert(usize::MAX);
            if new_dist <= *curr_entry {
                if new_dist == *curr_entry {
                    inbound_connections.entry((new_x, new_y, dir)).or_insert(Vec::new()).push((x, y, dir));
                } else {
                    let mut new_set = Vec::new();
                    new_set.push((x, y, dir));
                    inbound_connections.insert((new_x, new_y, dir), new_set);
                    *curr_entry = new_dist;
                }
                // println!("Updated: ({}, {}, {}): {}", new_x, new_y, dir, new_dist);
            }
        }

        node_dist.remove(&min_item.0);
    }

    // println!("{:?}", inbound_connections);
    let mut tiles_in_path = HashSet::new();
    count_path_tiles_from(start_x, start_y, RIGHT, &inbound_connections, &mut tiles_in_path);
    println!("Tiles in path: {}", tiles_in_path.len());

    let mut new_map = map.iter().map(|row| row.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    tiles_in_path.iter().for_each(|(x, y)| {
        new_map[*y][*x] = 'O';
    });
    // new_map.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>().iter().for_each(|row| println!("{}", row));

    Ok(())
}

fn count_path_tiles_from(start_x: usize, start_y: usize, dir: usize, connections: &HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>, tiles: &mut HashSet<(usize, usize)>) {
    connections.get(&(start_x, start_y, dir)).unwrap_or(&Vec::new()).iter().for_each(|(x, y, dir)| {
        tiles.insert((*x, *y));
        count_path_tiles_from(*x, *y, *dir, connections, tiles);
    });
}