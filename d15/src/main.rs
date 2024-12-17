use std::collections::{HashMap};
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("GPS sum 1: {}", part_1(&contents));

    println!("GPS sum 2: {}", part_2(&contents));

    Ok(())
}

fn part_1(contents: &str) -> usize {
    let mut map = Vec::new();
    let mut map_mode = true;
    let mut pos = (0, 0);
    contents.lines().enumerate().for_each(|(row, line)| {
        if line.is_empty() {
            map_mode = false;
            return;
        }
        if map_mode {
            map.push(line.chars().collect());
            for i in 0..line.len() {
                if line.chars().nth(i).unwrap() == '@' {
                    pos = (i as i32, row as i32);
                    break;
                }
            }
        } else {
            line.chars().for_each(|c| {
                // println!("{}", c);
                match c {
                    '^' => attempt_move_part_1(&mut pos, &mut map, 0, -1),
                    'v' => attempt_move_part_1(&mut pos, &mut map, 0, 1),
                    '<' => attempt_move_part_1(&mut pos, &mut map, -1, 0),
                    '>' => attempt_move_part_1(&mut pos, &mut map, 1, 0),
                    _ => unreachable!("Invalid character")
                }

                // map.iter().for_each(|row| {
                //     row.iter().for_each(|c| print!("{}", c));
                //     println!();
                // });
                // println!("--------------------");
            });
        }
    });

    let mut gps_sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                gps_sum += 100 * i + j;
            }
        }
    }

    gps_sum
}

fn attempt_move_part_1(pos: &mut (i32, i32), map: &mut Vec<Vec<char>>, dx: i32, dy: i32) {
    let mut box_x = -1;
    let mut box_y = -1;
    let (mut x, mut y) = *pos;
    // println!("{} {}", x, y);
    x += dx;
    y += dy;
    while map[y as usize][x as usize] == 'O' {
        if box_x == -1 {
            box_x = x;
            box_y = y;
        }
        x += dx;
        y += dy;
    }
    if map[y as usize][x as usize] == '#' {
        return;
    }
    if box_x == -1 {
        map[y as usize][x as usize] = '@';
        map[pos.1 as usize][pos.0 as usize] = '.';
        *pos = (x, y);
    } else {
        map[box_y as usize][box_x as usize] = '@';
        map[y as usize][x as usize] = 'O';
        *pos = (box_x, box_y);
    }
}

fn part_2(contents: &str) -> usize {
    let mut map = Vec::new();
    let mut map_mode = true;
    let mut pos = (0, 0);
    contents.lines().enumerate().for_each(|(row, line)| {
        if line.is_empty() {
            map_mode = false;
            return;
        }
        if map_mode {
            let mut chars = Vec::new();
            line.chars().for_each(|c| {
                match c {
                    '#' => {
                        chars.push('#');
                        chars.push('#');
                    },
                    '.' => {
                        chars.push('.');
                        chars.push('.');
                    },
                    '@' => {
                        chars.push('@');
                        chars.push('.');
                    },
                    'O' => {
                        chars.push('[');
                        chars.push(']');
                    },
                    _ => unreachable!("Invalid character")
                };
            });
            for i in 0..chars.len() {
                if chars[i] == '@' {
                    pos = (i as i32, row as i32);
                    break;
                }
            }
            map.push(chars);
        } else {
            line.chars().for_each(|c| {
                // println!("{}", c);
                match c {
                    '^' => attempt_move_part_2(&mut pos, &mut map, 0, -1),
                    'v' => attempt_move_part_2(&mut pos, &mut map, 0, 1),
                    '<' => attempt_move_part_2(&mut pos, &mut map, -1, 0),
                    '>' => attempt_move_part_2(&mut pos, &mut map, 1, 0),
                    _ => unreachable!("Invalid character")
                }

                // map.iter().for_each(|row| {
                //     row.iter().for_each(|c| print!("{}", c));
                //     println!();
                // });
                // println!("--------------------");
            });
        }
    });

    let mut gps_sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '[' {
                gps_sum += 100 * i + j;
            }
        }
    }

    gps_sum
}

fn attempt_move_part_2(pos: &mut (i32, i32), map: &mut Vec<Vec<char>>, dx: i32, dy: i32) {
    let mut move_cache = HashMap::new();
    let box_x = pos.0 + dx;
    let box_y = pos.1 + dy;
    if map[box_y as usize][box_x as usize] == '[' || map[box_y as usize][box_x as usize] == ']' {
        // println!("Can move box: {}", can_move_box((box_x as usize, box_y as usize), map, dx, dy, &mut move_cache));
        if can_move_box((box_x as usize, box_y as usize), map, dx, dy, &mut move_cache) {
            move_box((box_x as usize, box_y as usize), map, dx, dy);
            map[box_y as usize][box_x as usize] = '@';
            map[pos.1 as usize][pos.0 as usize] = '.';
            pos.0 = box_x;
            pos.1 = box_y;
        }
    } else if map[box_y as usize][box_x as usize] == '#' {
        return;
    } else {
        map[box_y as usize][box_x as usize] = '@';
        map[pos.1 as usize][pos.0 as usize] = '.';
        pos.0 = box_x;
        pos.1 = box_y;
    }
}

fn can_move_box(box_pos: (usize, usize), map: &mut Vec<Vec<char>>, dx: i32, dy: i32, cache: &mut HashMap<(usize, usize, i32, i32), bool>) -> bool {
    if cache.contains_key(&(box_pos.0, box_pos.1, dx, dy)) {
        return *cache.get(&(box_pos.0, box_pos.1, dx, dy)).unwrap();
    }

    if dx != 0 {
        let mut curr_x = box_pos.0;
        loop {
            curr_x = (curr_x as i32 + dx) as usize;
            match map[box_pos.1][curr_x] {
                '#' => {
                    cache.insert((box_pos.0, box_pos.1, dx, dy), false);
                    return false
                },
                '.' => {
                    cache.insert((box_pos.0, box_pos.1, dx, dy), true);
                    return true
                },
                _ => ()
            }
        }
    } else {
        // Vertical move
        let curr_char = map[box_pos.1][box_pos.0];
        let new_y = (box_pos.1 as i32 + dy) as usize;
        match curr_char {
            '[' => {
                let left_ok = match map[new_y][box_pos.0] {
                    '#' => false,
                    '.' => true,
                    '[' | ']' => can_move_box((box_pos.0, new_y), map, dx, dy, cache),
                    _ => unreachable!("Invalid character")
                };
                // Also check right side
                let right_ok = match map[new_y][box_pos.0 + 1] {
                    '#' => false,
                    '.' => true,
                    '[' | ']' => can_move_box((box_pos.0 + 1, new_y), map, dx, dy, cache),
                    _ => unreachable!("Invalid character")
                };
                cache.insert((box_pos.0, box_pos.1, dx, dy), left_ok && right_ok);
                left_ok && right_ok
            },
            ']' => {
                // Also check left side
                let left_ok = match map[new_y][box_pos.0 - 1] {
                    '#' => false,
                    '.' => true,
                    '[' | ']' => can_move_box((box_pos.0 - 1, new_y), map, dx, dy, cache),
                    _ => unreachable!("Invalid character")
                };
                let right_ok = match map[new_y][box_pos.0] {
                    '#' => false,
                    '.' => true,
                    '[' | ']' => can_move_box((box_pos.0, new_y), map, dx, dy, cache),
                    _ => unreachable!("Invalid character")
                };
                cache.insert((box_pos.0, box_pos.1, dx, dy), left_ok && right_ok);
                left_ok && right_ok
            },
            _ => unreachable!("Not a box")
        }
    }
}

fn move_box(box_pos: (usize, usize), map: &mut Vec<Vec<char>>, dx: i32, dy: i32) {
    if map[box_pos.1][box_pos.0] == '#' {
        panic!("Invalid wall position");
    }
    if map[box_pos.1][box_pos.0] == '.' {
        return
    }

    if dx != 0 {
        let new_x = (box_pos.0 as i32 + dx) as usize;
        move_box((new_x, box_pos.1), map, dx, dy);
        map[box_pos.1][new_x] = map[box_pos.1][box_pos.0];
        map[box_pos.1][box_pos.0] = '.';
    } else {
        // Vertical move
        let curr_char = map[box_pos.1][box_pos.0];
        let new_y = (box_pos.1 as i32 + dy) as usize;
        match curr_char {
            '[' => {
                move_box((box_pos.0, new_y), map, dx, dy);
                move_box((box_pos.0 + 1, new_y), map, dx, dy);
                map[new_y][box_pos.0] = '[';
                map[new_y][box_pos.0 + 1] = ']';
                map[box_pos.1][box_pos.0] = '.';
                map[box_pos.1][box_pos.0 + 1] = '.';
            },
            ']' => {
                move_box((box_pos.0 - 1, new_y), map, dx, dy);
                move_box((box_pos.0, new_y), map, dx, dy);
                map[new_y][box_pos.0 - 1] = '[';
                map[new_y][box_pos.0] = ']';
                map[box_pos.1][box_pos.0 - 1] = '.';
                map[box_pos.1][box_pos.0] = '.';
            },
            _ => unreachable!("Not a box")
        }
    }
}