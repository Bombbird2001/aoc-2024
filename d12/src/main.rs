use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let map = contents.lines().collect::<Vec<&str>>();
    let mut checked = map.iter().map(|line| line.chars().map(|_c| false).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();
    let mut price = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let (area, perimeter) = explore_from(i, j, &map, &mut checked);
            price += area * perimeter;
        }
    }

    println!("Price: {}", price);

    println!("New price: {}", new_price(&map));

    Ok(())
}

fn explore_from(row: usize, col: usize, map: &Vec<&str>, checked: &mut Vec<Vec<bool>>) -> (usize, usize) {
    if checked[row][col] {
        return (0, 0);
    }

    checked[row][col] = true;

    let (mut area, mut perimeter) = (1, 4);
    let this_plant = map[row].chars().nth(col).unwrap();

    if row >= 1 && map[row - 1].chars().nth(col).unwrap() == this_plant {
        let (a, p) = explore_from(row - 1, col, map, checked);
        area += a;
        perimeter += p;
        perimeter -= 1;
    }
    if row < map.len() - 1 && map[row + 1].chars().nth(col).unwrap() == this_plant {
        let (a, p) = explore_from(row + 1, col, map, checked);
        area += a;
        perimeter += p;
        perimeter -= 1;
    }
    if col >= 1 && map[row].chars().nth(col - 1).unwrap() == this_plant {
        let (a, p) = explore_from(row, col - 1, map, checked);
        area += a;
        perimeter += p;
        perimeter -= 1;
    }
    if col < map[row].len() - 1 && map[row].chars().nth(col + 1).unwrap() == this_plant {
        let (a, p) = explore_from(row, col + 1, map, checked);
        area += a;
        perimeter += p;
        perimeter -= 1;
    }

    (area, perimeter)
}

fn new_price(map: &Vec<&str>) -> usize {
    let map = dedup(map);

    let mut area_map = HashMap::new();
    map.iter().for_each(|line| line.iter().for_each(|c| *area_map.entry(c).or_insert(0usize) += 1));

    let mut segments_map: HashMap<usize, usize> = HashMap::new();
    // Iterate across rows first
    for i in 0..map.len() + 1 {
        let mut last_left_plant = 0;
        let mut last_right_plant = 0;
        for j in 0..map[0].len() {
            if i == 0 || i == map.len() || map[i - 1][j] != map[i][j] {
                // Both sides are different, check if segment needs to be added
                if i > 0 && map[i - 1][j] != last_left_plant {
                    last_left_plant = map[i - 1][j];
                    *segments_map.entry(last_left_plant).or_insert(0) += 1;
                }
                if i < map.len() && map[i][j] != last_right_plant {
                    last_right_plant = map[i][j];
                    *segments_map.entry(last_right_plant).or_insert(0) += 1;
                }
            } else {
                last_left_plant = 0;
                last_right_plant = 0;
            }
        }
    }

    // Iterate across columns
    for i in 0..map[0].len() + 1 {
        let mut last_left_plant = 0;
        let mut last_right_plant = 0;
        for j in 0..map.len() {
            if i == 0 || i == map[0].len() || map[j][i - 1] != map[j][i] {
                // Both sides are different, check if segment needs to be added
                if i > 0 && map[j][i - 1] != last_left_plant {
                    last_left_plant = map[j][i - 1];
                    *segments_map.entry(last_left_plant).or_insert(0) += 1;
                }
                if i < map[0].len() && map[j][i] != last_right_plant {
                    last_right_plant = map[j][i];
                    *segments_map.entry(last_right_plant).or_insert(0) += 1;
                }
            } else {
                last_left_plant = 0;
                last_right_plant = 0;
            }
        }
    }

    let price = area_map.iter().fold(0usize, |acc, (k, v)| acc + v * *segments_map.entry(**k).or_insert(0));

    price
}

fn dedup(map: &Vec<&str>) -> Vec<Vec<usize>> {
    let mut dedup_map = map.iter().map(|line| line.chars().map(|_c| 0).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();
    let mut next_id = 1;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if dedup_map[i][j] != 0 {
                continue;
            }

            mark_and_explore(next_id, i, j, &mut dedup_map, map);
            next_id += 1;
        }
    }

    dedup_map
}

fn mark_and_explore(new_id: usize, row: usize, col: usize, dedup_map: &mut Vec<Vec<usize>>, map: &Vec<&str>) {
    if dedup_map[row][col] != 0 {
        return;
    }
    dedup_map[row][col] = new_id;

    let this_ltr = map[row].chars().nth(col).unwrap();
    if row > 0 && this_ltr == map[row - 1].chars().nth(col).unwrap() {
        mark_and_explore(new_id, row - 1, col, dedup_map, map);
    }
    if row < map.len() - 1 && this_ltr == map[row + 1].chars().nth(col).unwrap() {
        mark_and_explore(new_id, row + 1, col, dedup_map, map);
    }
    if col > 0 && this_ltr == map[row].chars().nth(col - 1).unwrap() {
        mark_and_explore(new_id, row, col - 1, dedup_map, map);
    }
    if col < map[row].len() - 1 && this_ltr == map[row].chars().nth(col + 1).unwrap() {
        mark_and_explore(new_id, row, col + 1, dedup_map, map);
    }
}