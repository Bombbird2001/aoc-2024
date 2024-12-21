use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{Read, Result};

const INTERMEDIATE_ROBOTS: usize = 25;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let number_keypad_positions = HashMap::from(
    [
            ('7', (0i32, 0i32)), ('8', (1, 0)), ('9', (2, 0)),
            ('4', (0, 1)), ('5', (1, 1)), ('6', (2, 1)),
            ('1', (0, 2)), ('2', (1, 2)), ('3', (2, 2)),
            ('0', (1, 3)), ('A', (2, 3))
        ]);
    let number_key_index_mapping = HashMap::from(
        [
            ('7', 0), ('8', 1), ('9', 2), ('4', 3), ('5', 4), ('6', 5), ('1', 6), ('2', 7), ('3', 8), ('0', 9), ('A', 10)
        ]);

    let dir_keypad_positions = HashMap::from(
        [
            ('^', (1i32, 0i32)), ('>', (2, 1)), ('v', (1, 1)), ('<', (0, 1)), ('A', (2, 0))
        ]);
    let dir_key_index_mapping = HashMap::from(
        [
            ('^', 0), ('>', 1), ('v', 2), ('<', 3), ('A', 4)
        ]);

    let presses = compute_presses(&dir_keypad_positions, &dir_key_index_mapping, INTERMEDIATE_ROBOTS, &number_keypad_positions, &number_key_index_mapping);

    let codes = contents.lines().map(|line| line).collect::<Vec<&str>>();
    let complexity = codes.iter().fold(0, |acc, code| {
        // acc + get_complexity(code, &number_keypad_positions, &dir_keypad_positions)
        let num_portion = &code[0..code.chars().count() - 1].parse::<u64>().unwrap();
        let mut sum = 0;
        for i in 0..code.chars().count() - 1 {
            let from_key = code.chars().nth(i).unwrap();
            let to_key = code.chars().nth(i + 1).unwrap();

            if i == 0 {
                sum += presses[*number_key_index_mapping.get(&'A').unwrap()][*number_key_index_mapping.get(&from_key).unwrap()];
            }

            sum += presses[*number_key_index_mapping.get(&from_key).unwrap()][*number_key_index_mapping.get(&to_key).unwrap()];
        }

        acc + sum * num_portion
    });

    println!("Total complexity: {}", complexity);

    Ok(())
}

fn get_complexity(code: &str, number_keypad: &HashMap<char, (i32, i32)>, dir_keypad: &HashMap<char, (i32, i32)>) -> i32 {
    let num_portion = &code[0..code.chars().count() - 1].parse::<i32>().unwrap();

    let mut queue = VecDeque::new();
    let mut pending_queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut steps = 0;

    // BFS - states are the current hand positions of the robot, and the codes entered
    queue.push_back(('A', 'A', 'A', ['-', '-', '-', '-']));
    visited.insert(('A', 'A', 'A', ['-', '-', '-', '-']));
    while !queue.is_empty() {
        let (robot_1, robot_2, robot_3, code_state) = queue.pop_front().unwrap();

        if code_state == [code.chars().nth(0).unwrap(), code.chars().nth(1).unwrap(), code.chars().nth(2).unwrap(), code.chars().nth(3).unwrap()] {
            // println!("End reached");
            break;
        }

        if robot_1 == 'A' {
            // Robot 1 presses A on robot 2

            if robot_2 == 'A' {
                // Robot 2 presses A on robot 3

                // Robot 3 presses number on keypad, adds to first empty code slot
                for i in 0..4 {
                    if code_state[i] == '-' {
                        let mut new_code_state = code_state;
                        new_code_state[i] = robot_3;
                        check_add_node(&mut pending_queue, &mut visited, (robot_1, robot_2, robot_3, new_code_state));
                        break;
                    }
                }
            } else {
                // Robot 2 presses direction on robot 3, moves robot 3 hand
                // println!("Prev robot 3 pos: {}, press {}", robot_3, robot_2);
                let next_robot_3_pos = get_next_hand_position(robot_3, robot_2, number_keypad);
                if let Some(new_hand_pos) = next_robot_3_pos {
                    // println!("New robot 3 pos: {}", new_hand_pos);
                    check_add_node(&mut pending_queue, &mut visited, (robot_1, robot_2, new_hand_pos, code_state));
                }
            }
        } else {
            // Robot 1 presses direction on robot 2, moves robot 2 hand
            // println!("Prev robot 2 pos: {}, press {}", robot_2, robot_1);
            let next_robot_2_pos = get_next_hand_position(robot_2, robot_1, dir_keypad);
            if let Some(new_hand_pos) = next_robot_2_pos {
                // println!("New robot 2 pos: {}", new_hand_pos);
                check_add_node(&mut pending_queue, &mut visited, (robot_1, new_hand_pos, robot_3, code_state));
            }
        }

        // Can also move robot 1 hand to somewhere else
        // println!("Prev robot 1 pos: {}", robot_1);
        for manual_press_pos in ['^', '>', 'v', '<'].iter() {
            let next_robot_1_pos = get_next_hand_position(robot_1, *manual_press_pos, dir_keypad);
            if let Some(new_hand_pos) = next_robot_1_pos {
                // println!("New robot 1 pos: {}", new_hand_pos);
                check_add_node(&mut pending_queue, &mut visited, (new_hand_pos, robot_2, robot_3, code_state));
            }
        }

        if queue.is_empty() {
            std::mem::swap(&mut queue, &mut pending_queue);
            steps += 1;
        }
    }

    // println!("{} {}", steps, num_portion);

    steps * num_portion
}

fn check_add_node(
    pending_queue: &mut VecDeque<(char, char, char, [char; 4])>,
    visited: &mut HashSet<(char, char, char, [char; 4])>, node: (char, char, char, [char; 4])
) {
    if !visited.contains(&node) {
        pending_queue.push_back(node);
        visited.insert(node);
    }
}

fn get_next_hand_position(curr_hand_pos: char, dir_keypad_pressed: char, keypad_positions: &HashMap<char, (i32, i32)>) -> Option<char> {
    let (curr_x, curr_y) = keypad_positions.get(&curr_hand_pos).unwrap();

    let (new_x, new_y) = match dir_keypad_pressed {
        '^' => (*curr_x, curr_y - 1),
        '>' => (curr_x + 1, *curr_y),
        'v' => (*curr_x, curr_y + 1),
        '<' => (curr_x - 1, *curr_y),
        _ => unreachable!("Invalid direction keypad pressed")
    };

    keypad_positions.iter().find(|(_, &pos)| pos == (new_x, new_y)).and_then(|(k, _)| Some(*k))
}

fn compute_presses(dir_keypad: &HashMap<char, (i32, i32)>, dir_index_mapping: &HashMap<char, usize>, max_depth: usize,
                   number_keypad: &HashMap<char, (i32, i32)>, number_index_mapping: &HashMap<char, usize>) -> Vec<Vec<u64>> {
    // presses[robots_before][from_key][to_key]
    let mut dir_presses = vec![vec![vec![0u64; dir_keypad.len()]; dir_keypad.len()]; max_depth];

    for before_robots in 0..max_depth {
        dir_index_mapping.iter().for_each(|(from_key, &from_index)| {
            dir_index_mapping.iter().for_each(|(to_key, &to_index)| {
                if from_key == to_key {
                    dir_presses[before_robots][from_index][to_index] = 0;
                } else {
                    let (from_x, from_y) = dir_keypad.get(from_key).unwrap();
                    let (to_x, to_y) = dir_keypad.get(to_key).unwrap();
                    if before_robots == 0 {
                        let distance = ((from_x - to_x).abs() + (from_y - to_y).abs()) as u64;
                        dir_presses[before_robots][from_index][to_index] = distance;
                    } else {
                        let horiz_key = if from_x > to_x { '<' } else { '>' };
                        let vert_key = if from_y > to_y { '^' } else { 'v' };

                        let mut min_dist = None;

                        // First traverse x, then y
                        let turning_point = dir_keypad.iter().find(|(_, &(x, y))| x == *to_x && y == *from_y);
                        if turning_point.is_some() {
                            let dist = if to_x == from_x {
                                // Calculate A to vertical key, vertical key to A
                                dir_presses[before_robots - 1][*dir_index_mapping.get(&'A').unwrap()][*dir_index_mapping.get(&vert_key).unwrap()] + (to_y - from_y).abs() as u64
                                    + dir_presses[before_robots - 1][*dir_index_mapping.get(&vert_key).unwrap()][*dir_index_mapping.get(&'A').unwrap()]
                            } else {
                                // Calculate A to horizontal key, horizontal key to vertical key, vertical key to A
                                dir_presses[before_robots - 1][*dir_index_mapping.get(&'A').unwrap()][*dir_index_mapping.get(&horiz_key).unwrap()] + (to_x - from_x).abs() as u64
                                    + dir_presses[before_robots - 1][*dir_index_mapping.get(&horiz_key).unwrap()][*dir_index_mapping.get(&vert_key).unwrap()] + (to_y - from_y).abs() as u64
                                    + dir_presses[before_robots - 1][*dir_index_mapping.get(&vert_key).unwrap()][*dir_index_mapping.get(&'A').unwrap()]
                            };
                            min_dist = Some(dist);
                        }

                        // Then traverse y, then x
                        let turning_point = dir_keypad.iter().find(|(_, &(x, y))| x == *from_x && y == *to_y);
                        if turning_point.is_some() {
                            let dist = if to_y == from_y {
                                // Calculate A to horizontal key, horizontal key to A
                                dir_presses[before_robots - 1][*dir_index_mapping.get(&'A').unwrap()][*dir_index_mapping.get(&horiz_key).unwrap()] + (to_x - from_x).abs() as u64
                                    + dir_presses[before_robots - 1][*dir_index_mapping.get(&horiz_key).unwrap()][*dir_index_mapping.get(&'A').unwrap()]
                            } else {
                                // Calculate A to vertical key, vertical key to horizontal key, horizontal key to A
                                dir_presses[before_robots - 1][*dir_index_mapping.get(&'A').unwrap()][*dir_index_mapping.get(&vert_key).unwrap()] + (to_y - from_y).abs() as u64
                                    + dir_presses[before_robots - 1][*dir_index_mapping.get(&vert_key).unwrap()][*dir_index_mapping.get(&horiz_key).unwrap()] + (to_x - from_x).abs() as u64
                                    + dir_presses[before_robots - 1][*dir_index_mapping.get(&horiz_key).unwrap()][*dir_index_mapping.get(&'A').unwrap()]
                            };
                            if min_dist.is_none() || dist < min_dist.unwrap() {
                                min_dist = Some(dist);
                            }
                        }

                        dir_presses[before_robots][from_index][to_index] = min_dist.unwrap();
                    }
                }
            });
        });
    }

    let mut number_presses = vec![vec![0u64; number_keypad.len()]; number_keypad.len()];
    number_index_mapping.iter().for_each(|(from_key, &from_index)| {
        number_index_mapping.iter().for_each(|(to_key, &to_index)| {
            if from_key == to_key {
                number_presses[from_index][to_index] = 0;
            } else {
                let (from_x, from_y) = number_keypad.get(from_key).unwrap();
                let (to_x, to_y) = number_keypad.get(to_key).unwrap();

                let horiz_key = if from_x > to_x { '<' } else { '>' };
                let vert_key = if from_y > to_y { '^' } else { 'v' };

                let mut min_dist = None;

                // First traverse x, then y
                let turning_point = number_keypad.iter().find(|(_, &(x, y))| x == *to_x && y == *from_y);
                if turning_point.is_some() {
                    let dist = if to_x == from_x {
                        // Calculate A to vertical key, vertical key to A
                        dir_presses[max_depth - 1][*dir_index_mapping.get(&'A').unwrap()][*dir_index_mapping.get(&vert_key).unwrap()] + (to_y - from_y).abs() as u64
                            + dir_presses[max_depth - 1][*dir_index_mapping.get(&vert_key).unwrap()][*dir_index_mapping.get(&'A').unwrap()] + 1
                    } else {
                        // Calculate A to horizontal key, horizontal key to vertical key, vertical key to A
                        dir_presses[max_depth - 1][*dir_index_mapping.get(&'A').unwrap()][*dir_index_mapping.get(&horiz_key).unwrap()] + (to_x - from_x).abs() as u64
                            + dir_presses[max_depth - 1][*dir_index_mapping.get(&horiz_key).unwrap()][*dir_index_mapping.get(&vert_key).unwrap()] + (to_y - from_y).abs() as u64
                            + dir_presses[max_depth - 1][*dir_index_mapping.get(&vert_key).unwrap()][*dir_index_mapping.get(&'A').unwrap()] + 1
                    };
                    min_dist = Some(dist);
                }

                // Then traverse y, then x
                let turning_point = number_keypad.iter().find(|(_, &(x, y))| x == *from_x && y == *to_y);
                if turning_point.is_some() {
                    let dist = if to_y == from_y {
                        // Calculate A to horizontal key, horizontal key to A
                        dir_presses[max_depth - 1][*dir_index_mapping.get(&'A').unwrap()][*dir_index_mapping.get(&horiz_key).unwrap()] + (to_x - from_x).abs() as u64
                            + dir_presses[max_depth - 1][*dir_index_mapping.get(&horiz_key).unwrap()][*dir_index_mapping.get(&'A').unwrap()] + 1
                    } else {
                        // Calculate A to vertical key, vertical key to horizontal key, horizontal key to A
                        dir_presses[max_depth - 1][*dir_index_mapping.get(&'A').unwrap()][*dir_index_mapping.get(&vert_key).unwrap()] + (to_y - from_y).abs() as u64
                            + dir_presses[max_depth - 1][*dir_index_mapping.get(&vert_key).unwrap()][*dir_index_mapping.get(&horiz_key).unwrap()] + (to_x - from_x).abs() as u64
                            + dir_presses[max_depth - 1][*dir_index_mapping.get(&horiz_key).unwrap()][*dir_index_mapping.get(&'A').unwrap()] + 1
                    };
                    if min_dist.is_none() || dist < min_dist.unwrap() {
                        min_dist = Some(dist);
                    }
                }

                number_presses[from_index][to_index] = min_dist.unwrap();
            }
        });
    });

    number_presses
}