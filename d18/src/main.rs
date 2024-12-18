use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Result};
use std::mem;

const DIMENSIONS: usize = 71;
const LIMIT: usize = 1024;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut map = vec![vec!['.'; DIMENSIONS]; DIMENSIONS];
    contents.lines().take(LIMIT).for_each(|line| {
        let strs = line.split(",").collect::<Vec<&str>>();
        let x = strs[0].parse::<usize>().unwrap();
        let y = strs[1].parse::<usize>().unwrap();
        map[x][y] = '#';
    });

    let dist = bfs(&map).unwrap();
    println!("Distance: {}", dist);

    let lines = contents.lines().collect::<Vec<&str>>();

    for i in LIMIT..lines.len() {
        let strs = lines[i].split(",").collect::<Vec<&str>>();
        let x = strs[0].parse::<usize>().unwrap();
        let y = strs[1].parse::<usize>().unwrap();
        map[x][y] = '#';
        let dist = bfs(&map);
        if dist.is_none() {
            println!("First block: {}, {}", x, y);
            break;
        }
    }

    Ok(())
}

fn bfs(map: &Vec<Vec<char>>) -> Option<usize> {
    // BFS
    let mut queue = VecDeque::new();
    let mut pending_queue = VecDeque::new();
    let mut dist = 0;
    let mut visited = vec![vec![false; DIMENSIONS]; DIMENSIONS];
    queue.push_back((0, 0));
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if x == DIMENSIONS - 1 && y == DIMENSIONS - 1 {
            return Some(dist);
        }
        if x > 0 && map[x - 1][y] == '.' && !visited[x - 1][y] {
            visited[x - 1][y] = true;
            pending_queue.push_back((x - 1, y));
        }
        if y > 0 && map[x][y - 1] == '.' && !visited[x][y - 1] {
            visited[x][y - 1] = true;
            pending_queue.push_back((x, y - 1));
        }
        if x < DIMENSIONS - 1 && map[x + 1][y] == '.' && !visited[x + 1][y] {
            visited[x + 1][y] = true;
            pending_queue.push_back((x + 1, y));
        }
        if y < DIMENSIONS - 1 && map[x][y + 1] == '.' && !visited[x][y + 1] {
            visited[x][y + 1] = true;
            pending_queue.push_back((x, y + 1));
        }
        if queue.is_empty() {
            mem::swap(&mut queue, &mut pending_queue);
            dist += 1;
        }
    }

    None
}